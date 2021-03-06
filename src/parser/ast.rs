#[derive(Debug, PartialEq)]
pub enum MonkeyCStatement {
    VariableDeclaration {
        name: String,
        default_val: MonkeyCExpression,
        var_type: Option<String>,
        is_const: bool
    },
    ClassDeclaration {
        name: String,
        extends: Option<String>,
        children: Vec<MonkeyCStatement>,
    }
    // Others will be added in later
}

#[derive(Debug, PartialEq)]
pub enum MonkeyCExpression {
    /// "Simple" assignment
    /// # Example
    /// ```
    /// var myVar = "simple string"
    /// ```
    Simple(String),
    /// Reference to other variables/functions/classes/etc
    ///
    /// # Examples
    /// ```
    /// var fooVar = fooFunc();
    /// var barVar = new BarClass();
    /// var bazVar = otherBazVar;
    /// ```
    Reference(String),
    /// Mathematical equations.
    /// # Examples
    /// ```
    /// var fooVar = 1 + 2;
    /// var barVar = fooVar * 2;
    /// var bazVar = barVar / 2;
    /// ```
    Mathematical(Box<MonkeyCExpression>, MonkeyCExprMathOperation, Box<MonkeyCExpression>),
    /// Binary operations.
    /// # Examples
    /// ```
    /// var fooVar = 1 <= 2;
    /// if (fooVar) { // Should be true, otherwise the language broke
    ///     // Do smth
    /// }
    /// ```
    Binary(Box<MonkeyCExpression>, MonkeyCExprBinaryOperation, Box<MonkeyCExpression>),
    /// Bitwise operations.
    /// # Examples
    /// ```
    /// var fooVar = 0x111 << 0x001;
    /// ```
    Bitwise(Box<MonkeyCExpression>, MonkeyCExprBitwiseOperation, Box<MonkeyCExpression>)
}

#[derive(Debug, PartialEq)]
pub enum MonkeyCExprBitwiseOperation {
    LeftShift,
    RightShift,
    And,
    Or,
    Xor
}

#[derive(Debug, PartialEq)]
pub enum MonkeyCExprBinaryOperation {
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Equals,
    NotEquals
}

#[derive(Debug, PartialEq)]
pub enum MonkeyCExprMathOperation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulo,
}
