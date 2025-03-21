//! # DAE: Differential Algebraic Equations
//!
//! v := [p; t; x_dot; x; y; z; m; pre(z); pre(m)]
//!
//! 0 = fx(v, c)
//!
//! z = {
//!     fz(v, c) at events
//!     pre(z)   otherwise
//! }
//!
//! m := fm(v, c)
//!
//! c := fc(relation, v)
//!
//! ### where:
//!
//! * `p`           : Modelica variables declared as parameters or constants,
//!                   i.e., variables without any time-dependency.
//! * `t`           : Modelica variable representing time, the independent (real) variable.
//! * `x(t)`        : Modelica variables of type `Real` that appear differentiated.
//! * `y(t)`        : Continuous-time Modelica variables of type `Real` that do not
//!                   appear differentiated (algebraic variables).
//! * `z(t_e)`      : Discrete-time Modelica variables of type `Real`. These
//!                   variables change their value only at event instants `t_e`. `pre(z)`
//!                   represents the values immediately before the current event occurred.
//! * `m(t_e)`      : Modelica variables of discrete-valued types (Boolean,
//!                   Integer, etc) which are unknown. These variables change their value
//!                   only at event instants
//! * `c(t_e)`      : The conditions of all if-expressions generated including
//!                   when-clauses after conversion, see section 8.3.5).
//! * `relation(v)` : A relation containing variables v_i, e.g. v1 > v2, v3 >= 0.
//!                   algebraic equations

use crate::ir::ast::{Component, Equation, Expression};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dae {
    pub p: Vec<Component>,     // parameters
    pub cp: Vec<Component>,    // constant parameters (ADDED)
    pub t: Component,          // time
    pub x: Vec<Component>,     // continous states
    pub x_dot: Vec<Component>, // derivatives of continuous states
    pub y: Vec<Component>,     // alg. variables
    pub u: Vec<Component>,     // input (ADDED)
    pub pre_z: Vec<Component>, // z before event time t_e
    pub z: Vec<Component>,     // real discrete variables, only change at t_e
    pub m: Vec<Component>,     // variables of discrete-value types, only change at t_e
    pub c: Vec<Expression>,        // conditions of all if-expressions/ when-clauses
    pub relation: Vec<Expression>, // relations
    pub fx: Vec<Equation>,     // continuous time equations
    pub fz: Vec<Equation>,     // event update equations
    pub fm: Vec<Equation>,     // discrete update equations
    pub fc: Vec<Vec<Equation>>,  // callback for conditions
}
