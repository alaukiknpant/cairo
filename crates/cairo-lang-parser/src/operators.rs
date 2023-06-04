use cairo_lang_syntax::node::kind::SyntaxKind;

pub fn get_unary_operator_precedence(kind: SyntaxKind) -> Option<usize> {
    match kind {
        SyntaxKind::TerminalAt
        | SyntaxKind::TerminalNot
        | SyntaxKind::TerminalBitNot
        | SyntaxKind::TerminalMul => Some(2),
        SyntaxKind::TerminalMinus => Some(4),
        _ => None,
    }
}
pub fn get_post_operator_precedence(kind: SyntaxKind) -> Option<usize> {
    match kind {
        SyntaxKind::TerminalDot => Some(0),
        SyntaxKind::TerminalQuestionMark
        // [] Operator.
        | SyntaxKind::TerminalLBrack => Some(1),
        SyntaxKind::TerminalAt | SyntaxKind::TerminalNot => Some(2),
        SyntaxKind::TerminalMul | SyntaxKind::TerminalDiv | SyntaxKind::TerminalMod => Some(3),
        SyntaxKind::TerminalPlus | SyntaxKind::TerminalMinus => Some(4),
        SyntaxKind::TerminalAndAnd => Some(5),
        SyntaxKind::TerminalOrOr => Some(6),
        SyntaxKind::TerminalEqEq
        | SyntaxKind::TerminalNeq
        | SyntaxKind::TerminalLT
        | SyntaxKind::TerminalGT
        | SyntaxKind::TerminalLE
        | SyntaxKind::TerminalGE => Some(7),
        SyntaxKind::TerminalAnd => Some(8),
        SyntaxKind::TerminalOr => Some(9),
        SyntaxKind::TerminalXor => Some(10),
        SyntaxKind::TerminalEq
        | SyntaxKind::TerminalPlusEq
        | SyntaxKind::TerminalMinusEq
        | SyntaxKind::TerminalMulEq
        | SyntaxKind::TerminalDivEq
        | SyntaxKind::TerminalModEq => Some(11),
        _ => None,
    }
}
