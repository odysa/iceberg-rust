// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! This module contains expressions.

mod term;

use std::fmt::{Display, Formatter};

pub use term::*;
mod predicate;
pub use predicate::*;

/// Predicate operators used in expressions.
///
/// The discriminant of this enum is used for determining the type of the operator, see
/// [`PredicateOperator::is_unary`], [`PredicateOperator::is_binary`], [`PredicateOperator::is_set`]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum PredicateOperator {
    // Unary operators
    IsNull = 101,
    NotNull = 102,
    IsNan = 103,
    NotNan = 104,

    // Binary operators
    LessThan = 201,
    LessThanOrEq = 202,
    GreaterThan = 203,
    GreaterThanOrEq = 204,
    Eq = 205,
    NotEq = 206,
    StartsWith = 207,
    NotStartsWith = 208,

    // Set operators
    In = 301,
    NotIn = 302,
}

impl Display for PredicateOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PredicateOperator::IsNull => write!(f, "IS NULL"),
            PredicateOperator::NotNull => write!(f, "IS NOT NULL"),
            PredicateOperator::IsNan => write!(f, "IS NAN"),
            PredicateOperator::NotNan => write!(f, "IS NOT NAN"),
            PredicateOperator::LessThan => write!(f, "<"),
            PredicateOperator::LessThanOrEq => write!(f, "<="),
            PredicateOperator::GreaterThan => write!(f, ">"),
            PredicateOperator::GreaterThanOrEq => write!(f, ">="),
            PredicateOperator::Eq => write!(f, "="),
            PredicateOperator::NotEq => write!(f, "!="),
            PredicateOperator::In => write!(f, "IN"),
            PredicateOperator::NotIn => write!(f, "NOT IN"),
            PredicateOperator::StartsWith => write!(f, "STARTS WITH"),
            PredicateOperator::NotStartsWith => write!(f, "NOT STARTS WITH"),
        }
    }
}

impl PredicateOperator {
    /// Check if this operator is unary operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iceberg::expr::PredicateOperator;
    /// assert!(PredicateOperator::IsNull.is_unary());
    /// ```
    pub fn is_unary(self) -> bool {
        (self as u16) < (PredicateOperator::LessThan as u16)
    }

    /// Check if this operator is binary operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iceberg::expr::PredicateOperator;
    /// assert!(PredicateOperator::LessThan.is_binary());
    /// ```
    pub fn is_binary(self) -> bool {
        ((self as u16) > (PredicateOperator::NotNan as u16))
            && ((self as u16) < (PredicateOperator::In as u16))
    }

    /// Check if this operator is set operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iceberg::expr::PredicateOperator;
    /// assert!(PredicateOperator::In.is_set());
    /// ```
    pub fn is_set(self) -> bool {
        (self as u16) > (PredicateOperator::NotStartsWith as u16)
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::PredicateOperator;

    #[test]
    fn test_unary() {
        assert!(PredicateOperator::IsNull.is_unary());
        assert!(PredicateOperator::NotNull.is_unary());
        assert!(PredicateOperator::IsNan.is_unary());
        assert!(PredicateOperator::NotNan.is_unary());
    }

    #[test]
    fn test_binary() {
        assert!(PredicateOperator::LessThan.is_binary());
        assert!(PredicateOperator::LessThanOrEq.is_binary());
        assert!(PredicateOperator::GreaterThan.is_binary());
        assert!(PredicateOperator::GreaterThanOrEq.is_binary());
        assert!(PredicateOperator::Eq.is_binary());
        assert!(PredicateOperator::NotEq.is_binary());
        assert!(PredicateOperator::StartsWith.is_binary());
        assert!(PredicateOperator::NotStartsWith.is_binary());
    }

    #[test]
    fn test_set() {
        assert!(PredicateOperator::In.is_set());
        assert!(PredicateOperator::NotIn.is_set());
    }
}