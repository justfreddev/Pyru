#[macro_export]
macro_rules! arithmetic {
    ( $operator:tt ; $num1:expr ; $num2:expr ) => {
        if let Value::Literal(LiteralType::Num(ln)) = $num1 {
            if let Value::Literal(LiteralType::Num(rn)) = $num2 {
                return Err(Value::Literal(LiteralType::Num(ln $operator rn)));
            }
        }
    };
}

#[macro_export]
macro_rules! comparison {
    ( $operator:tt ; $num1:expr ; $num2:expr ) => {
        if let Value::Literal(LiteralType::Num(ln)) = $num1 {
            if let Value::Literal(LiteralType::Num(rn)) = $num2 {
                return if ln $operator rn { Err(Value::Literal(LiteralType::True)) } else { Err(Value::Literal(LiteralType::False)) }
            }
        }
    };
}

#[macro_export]
macro_rules! getresult {
    ( $result:ident ) => {
        match $result {
            Ok(_) => None,
            Err(v) => Some(v)
        }.unwrap()
    };
}

#[macro_export]
macro_rules! returncheck {
    ( $result:ident ) => {
        match $result {
            Err(v) => return Err(v),
            Ok(()) => {}
        }
    };
}

#[macro_export]
macro_rules! keywords {
    ( $kw:expr ; $($kws:ident),+ ) => {
        $(
            let key = stringify!($kws).to_lowercase();
            $kw.insert(key, TokenType::$kws);
        )+
    };
}

#[macro_export]
macro_rules! stmt_visitor {
    ( $($stmts:ident),+ ) => {
        pub trait StmtVisitor<T> {
            $(
                paste! {
                    fn [<visit_ $stmts:lower _stmt>](&mut self, stmt: &Stmt) -> T;
                }
            )+
        }
    
        impl Stmt {
            pub fn accept_stmt<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
                match self {
                    $(
                        Stmt::$stmts { .. } => {
                            paste! {
                                visitor.[<visit_ $stmts:lower _stmt>](self)
                            }
                        },
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! expr_visitor {
    ( $($exprs:ident),+ ) => {
        pub trait ExprVisitor<T> {
            $(
                paste! {
                    fn [<visit_ $exprs:lower _expr>](&mut self, stmt: &Expr) -> T;
                }
            )+
        }
    
        impl Expr {
            pub fn accept_expr<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
                match self {
                    $(
                        Expr::$exprs { .. } => {
                            paste! {
                                visitor.[<visit_ $exprs:lower _expr>](self)
                            }
                        },
                    )+
                }
            }
        }
    };
}