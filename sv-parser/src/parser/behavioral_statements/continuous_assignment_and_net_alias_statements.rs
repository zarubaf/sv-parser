use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum ContinuousAssign {
    Net(Box<ContinuousAssignNet>),
    Variable(Box<ContinuousAssignVariable>),
}

#[derive(Clone, Debug, Node)]
pub struct ContinuousAssignNet {
    pub nodes: (
        Keyword,
        Option<DriveStrength>,
        Option<Delay3>,
        ListOfNetAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct ContinuousAssignVariable {
    pub nodes: (
        Keyword,
        Option<DelayControl>,
        ListOfVariableAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfNetAssignments {
    pub nodes: (List<Symbol, NetAssignment>,),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfVariableAssignments {
    pub nodes: (List<Symbol, VariableAssignment>,),
}

#[derive(Clone, Debug, Node)]
pub struct NetAlias {
    pub nodes: (Keyword, NetLvalue, Symbol, List<Symbol, NetLvalue>, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct NetAssignment {
    pub nodes: (NetLvalue, Symbol, Expression),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn continuous_assign(s: Span) -> IResult<Span, ContinuousAssign> {
    alt((continuous_assign_net, continuous_assign_variable))(s)
}

#[parser]
pub(crate) fn continuous_assign_net(s: Span) -> IResult<Span, ContinuousAssign> {
    let (s, a) = keyword("assign")(s)?;
    let (s, b) = opt(drive_strength)(s)?;
    let (s, c) = opt(delay3)(s)?;
    let (s, d) = list_of_net_assignments(s)?;
    let (s, e) = symbol(";")(s)?;

    Ok((
        s,
        ContinuousAssign::Net(Box::new(ContinuousAssignNet {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[parser]
pub(crate) fn continuous_assign_variable(s: Span) -> IResult<Span, ContinuousAssign> {
    let (s, a) = keyword("assign")(s)?;
    let (s, b) = opt(delay_control)(s)?;
    let (s, c) = list_of_variable_assignments(s)?;
    let (s, d) = symbol(";")(s)?;

    Ok((
        s,
        ContinuousAssign::Variable(Box::new(ContinuousAssignVariable {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser(MaybeRecursive)]
pub(crate) fn list_of_net_assignments(s: Span) -> IResult<Span, ListOfNetAssignments> {
    let (s, a) = list(symbol(","), net_assignment)(s)?;
    Ok((s, ListOfNetAssignments { nodes: (a,) }))
}

#[parser(MaybeRecursive)]
pub(crate) fn list_of_variable_assignments(s: Span) -> IResult<Span, ListOfVariableAssignments> {
    let (s, a) = list(symbol(","), variable_assignment)(s)?;
    Ok((s, ListOfVariableAssignments { nodes: (a,) }))
}

#[parser]
pub(crate) fn net_alias(s: Span) -> IResult<Span, NetAlias> {
    let (s, a) = keyword("alias")(s)?;
    let (s, b) = net_lvalue(s)?;
    let (s, c) = symbol("=")(s)?;
    let (s, d) = list(symbol("="), net_lvalue)(s)?;
    let (s, e) = symbol(";")(s)?;

    Ok((
        s,
        NetAlias {
            nodes: (a, b, c, d, e),
        },
    ))
}

#[parser(MaybeRecursive)]
pub(crate) fn net_assignment(s: Span) -> IResult<Span, NetAssignment> {
    let (s, a) = net_lvalue(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = expression(s)?;

    Ok((s, NetAssignment { nodes: (a, b, c) }))
}