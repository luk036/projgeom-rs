
#[cfg(test)]
use core::hash;
// use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// use core::str::FromStr;
#[cfg(feature = "std")]
use std::error::Error;
use std::mem; // for swap
use num_integer::gcd;
use num_traits::{Num, Signed, Zero, One};

#[derive(Copy, Clone, Hash, Debug, Default)]
// #[repr(C)]
pub struct Fraction<Z> {
    /// numerator portion of the Fraction object
    pub num: Z,
    /// denominator portion of the Fraction object
    pub den: Z,
}

impl<Z: Num + Zero + One> Fraction<Z> {
    /// Create a new Fraction
    #[inline]
    pub fn new(num: Z, den: Z) -> Self {
        let mut res = Fraction { num, den };
        res.normalize();
        res
    }

    /**
     * @brief normalize to a canonical form
     *
     * denominator is always non-negative and co-prime with numerator
     */
    pub fn normalize(&mut self) -> Z {
        self.normalize1();
        self.normalize2()
    }

    /**
     * @brief normalize to a canonical form
     *
     * denominator is always co-prime with numerator
     */
    pub fn normalize2(&mut self) -> Z {
        let common: Z = gcd(self.num, self.den);
        if common != One::one() && common != Zero::zero() {
            self.num /= common;
            self.den /= common;
        }
        common
    }
}

impl<Z: Num + Zero> Fraction<Z> {
    /**
     * @brief normalize to a canonical form
     *
     * denominator is always non-negative
     */
    pub fn normalize1(&mut self) {
        if self.den < Zero::zero() {
            self.num = -self.num;
            self.den = -self.den;
        }
    }
}

impl<Z: Num + One> Fraction<Z> {
    #[inline]
    pub fn from(num: Z) -> Self {
        Fraction { num, den: One::one() }
    }
}

impl<Z: Num + One + Zero> Default for Fraction<Z> {
    #[inline]
    pub fn default() -> Self {
        Fraction { num: Zero::zero(), den: One::one() }
    }
}

impl<Z: Num> Fraction<Z> {
    /**
     * @brief cross product
     *
     * @param rhs
     * @return Z
     */
    pub fn cross(&self, rhs: &Fraction) -> Z {
        self.num * rhs.den - self.den * rhs.num
    }
}

impl<Z: Num + PartialEq + Clone> PartialEq<Rhs = Z> for Fraction<Z> {
    /** @name Comparison operators
     *  ==, !=, <, >, <=, >= etc.
     */
    ///@{

    /**
     * @brief Equal to
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn eq(&self, other: &Z) -> bool {
        if self.den == One::one() || rhs == Zero::zero() {
            return self.num == other;
        }
        let mut lhs = self.clone();
        let mut rhs = other.clone();
        mem::swap(&mut lhs.den, &mut rhs);
        lhs.normalize2();
        lhs.num == self.den * rhs;
    }
}
impl<Z: Num + Eq + Clone> Eq<Rhs = Z> for Fraction<Z> {}


impl<Z: Num + PartialOrd + Clone> PartialOrd<Rhs = Z> for Fraction<Z> {}
    /**
     * @brief Less than
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn lt(&self, other: &Z) -> bool {
        if self.den == One::one() || other == Zero::zero() {
            return self.num < other;
        }
        let mut lhs = self.clone();
        let mut rhs = other.clone();
        mem::swap(&mut lhs.den, &mut rhs.num);
        lhs.normalize2();
        lhs.num < lhs.den * rhs
    }

    /**
     * @brief Less than
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator<(Z lhs, Fraction rhs) -> bool {
        if rhs.den == One::one() || lhs == Zero::zero() {
            return lhs < rhs.num;
        }
        mem::swap(&mut rhs.den, &mut lhs);
        rhs.normalize2();
        return rhs.den * lhs < rhs.num;
    }

    /**
     * @brief Equal to
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator==(const Z& lhs, const Fraction& rhs) -> bool {
        return rhs == lhs;
    }

    /**
     * @brief Equal to
     *
     * @param[in] rhs
     * @return true
     * @return false
     */

    /**
     * @brief Equal to
     *
     * @param lhs
     * @param rhs
     * @return true
     * @return false
     */
    constexpr pub fn operator==(Fraction lhs, Fraction rhs) -> bool {
        if lhs.den == rhs.den {
            return lhs.num == rhs.num;
        }
        mem::swap(&mut lhs.den, &mut rhs.num);
        lhs.normalize2();
        rhs.normalize2();
        return lhs.num * rhs.den == lhs.den * rhs.num;
    }

    /**
     * @brief Less than
     *
     * @param lhs
     * @param rhs
     * @return true
     * @return false
     */
    constexpr pub fn operator<(Fraction lhs, Fraction rhs) -> bool {
        if lhs.den == rhs.den {
            return lhs.num < rhs.num;
        }
        mem::swap(&mut lhs.den, &mut rhs.num);
        lhs.normalize2();
        rhs.normalize2();
        return lhs.num * rhs.den < lhs.den * rhs.num;
    }

    /**
     * @brief
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator!=(const Fraction& rhs) const -> bool { return !(*this == rhs); }

    /**
     * @brief Greater than
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>(const Fraction& rhs) const -> bool { return rhs < *this; }

    /**
     * @brief Greater than or euqal to
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>=(const Fraction& rhs) const -> bool { return !(*this < rhs); }

    /**
     * @brief Less than or equal to
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator<=(const Fraction& rhs) const -> bool { return !(rhs < *this); }

    /**
     * @brief Greater than
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>(const Z& rhs) const -> bool { return rhs < *this; }

    /**
     * @brief Less than or equal to
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator<=(const Z& rhs) const -> bool { return !(rhs < *this); }

    /**
     * @brief Greater than or equal to
     *
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>=(const Z& rhs) const -> bool { return !(*this < rhs); }

    /**
     * @brief Greater than
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>(const Z& lhs, const Fraction& rhs) -> bool {
        return rhs < lhs;
    }

    /**
     * @brief Less than or equal to
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator<=(const Z& lhs, const Fraction& rhs) -> bool {
        return !(rhs < lhs);
    }

    /**
     * @brief Greater than or euqal to
     *
     * @param[in] lhs
     * @param[in] rhs
     * @return true
     * @return false
     */
    pub fn operator>=(const Z& lhs, const Fraction& rhs) -> bool {
        return !(lhs < rhs);
    }

    ///@}

    /**
     * @brief reciprocal
     *
     */
    pub fn reciprocal() noexcept(std::is_nothrow_swappable_v<Z>) {
        mem::swap(&mut self.num, &mut self.den);
        self.normalize1();
    }

    /**
     * @brief multiply and assign
     *
     * @param rhs
     * @return Fraction&
     */
    pub fn operator*=(Fraction rhs) -> Fraction& {
        mem::swap(&mut self.num, &mut rhs.num);
        self.normalize2();
        rhs.normalize2();
        self.num *= rhs.num;
        self.den *= rhs.den;
        return *this;
    }

    /**
     * @brief multiply
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator*(Fraction lhs, const Fraction& rhs) -> Fraction {
        return lhs *= rhs;
    }

    /**
     * @brief multiply and assign
     *
     * @param rhs
     * @return Fraction&
     */
    pub fn operator*=(Z rhs) -> Fraction& {
        mem::swap(&mut self.num, &mut rhs);
        self.normalize2();
        self.num *= rhs;
        return *this;
    }

    /**
     * @brief multiply
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator*(Fraction lhs, const Z& rhs) -> Fraction {
        return lhs *= rhs;
    }

    /**
     * @brief multiply
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator*(const Z& lhs, Fraction rhs) -> Fraction {
        return rhs *= lhs;
    }

    /**
     * @brief divide and assign
     *
     * @param rhs
     * @return Fraction&
     */
    pub fn operator/=(Fraction rhs) -> Fraction& {
        mem::swap(&mut self.den, &mut rhs.num);
        self.normalize();
        rhs.normalize2();
        self.num *= rhs.den;
        self.den *= rhs.num;
        return *this;
    }

    /**
     * @brief divide
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator/(Fraction lhs, const Fraction& rhs) -> Fraction {
        return lhs /= rhs;
    }

    /**
     * @brief divide and assign
     *
     * @param rhs
     * @return Fraction&
     */
    pub fn operator/=(const Z& rhs) -> Fraction& {
        mem::swap(&mut self.den, &mut rhs);
        self.normalize();
        self.den *= rhs;
        return *this;
    }

    /**
     * @brief divide
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator/(Fraction lhs, const Z& rhs) -> Fraction {
        return lhs /= rhs;
    }

    /**
     * @brief divide
     *
     * @param lhs
     * @param rhs
     * @return Fraction
     */
    pub fn operator/(const Z& lhs, Fraction rhs) -> Fraction {
        rhs.reciprocal();
        return rhs *= lhs;
    }

    /**
     * @brief Negate
     *
     * @return Fraction
     */
    pub fn operator-() const -> Fraction {
        let mut res = Fraction(*this);
        res.num = -res.num;
        return res;
    }

    /**
     * @brief Add
     *
     * @param rhs
     * @return Fraction
     */
    pub fn operator+(const Fraction& rhs) const -> Fraction {
        if self.den == rhs.den {
            return Fraction(self.num + rhs.num, self.den);
        }
        let common = gcd(self.den, rhs.den);
        if common == Zero::zero() {
            return Fraction(rhs.den * self.num + self.den * rhs.num, Zero::zero());
        }
        let l = self.den / common;
        let r = rhs.den / common;
        let mut d = self.den * r;
        let mut n = r * self.num + l * rhs.num;
        return Fraction(std::move(n), std::move(d));
    }

    /**
     * @brief Subtract
     *
     * @param[in] frac
     * @return Fraction
     */
    pub fn operator-(const Fraction& frac) const -> Fraction { return *this + (-frac); }

    /**
     * @brief Add
     *
     * @param[in] frac
     * @param[in] i
     * @return Fraction
     */
    pub fn operator+(Fraction frac, const Z& i) -> Fraction { return frac += i; }

    /**
     * @brief Add
     *
     * @param[in] i
     * @param[in] frac
     * @return Fraction
     */
    pub fn operator+(const Z& i, Fraction frac) -> Fraction { return frac += i; }

    /**
     * @brief
     *
     * @param[in] i
     * @return Fraction
     */
    pub fn operator-(const Z& i) const -> Fraction { return *this + (-i); }

    /**
     * @brief
     *
     * @param[in] rhs
     * @return Fraction
     */
    pub fn operator+=(const Fraction& rhs) -> Fraction& { return *this -= (-rhs); }

    /**
     * @brief
     *
     * @param[in] rhs
     * @return Fraction
     */
    pub fn operator-=(const Fraction& rhs) -> Fraction& {
        if self.den == rhs.den {
            self.num -= rhs.num;
            self.normalize2();
            return *this;
        }

        let mut other{rhs};
        mem::swap(&mut self.den, &mut other.num);
        let mut common_n = self.normalize2();
        let mut common_d = other.normalize2();
        mem::swap(&mut self.den, &mut other.num);
        self.num = self.cross(other);
        self.den *= other.den;
        mem::swap(&mut self.den, &mut common_d);
        self.normalize2();
        self.num *= common_n;
        self.den *= common_d;
        self.normalize2();
        return *this;
    }

    /**
     * @brief
     *
     * @param[in] i
     * @return Fraction
     */
    pub fn operator+=(const Z& i) -> Fraction& { return *this -= (-i); }

    /**
     * @brief
     *
     * @param[in] rhs
     * @return Fraction
     */
    pub fn operator-=(const Z& rhs) -> Fraction& {
        if self.den == One::one() {
            self.num -= rhs;
            return *this;
        }

        let mut other{rhs};
        mem::swap(&mut self.den, &mut other);
        let mut common_n = self.normalize2();
        mem::swap(&mut self.den, &mut other);
        self.num -= other * self.den;
        self.num *= common_n;
        self.normalize2();
        return *this;
    }

    /**
     * @brief
     *
     * @param[in] c
     * @param[in] frac
     * @return Fraction
     */
    pub fn operator-(const Z& c, const Fraction& frac) -> Fraction {
        return c + (-frac);
    }

    /**
     * @brief
     *
     * @param[in] c
     * @param[in] frac
     * @return Fraction
     */
    pub fn operator+(int&& c, const Fraction& frac) -> Fraction {
        return frac + Z(c);
    }

    /**
     * @brief
     *
     * @param[in] c
     * @param[in] frac
     * @return Fraction
     */
    pub fn operator-(int&& c, const Fraction& frac) -> Fraction {
        return (-frac) + Z(c);
    }

    /**
     * @brief
     *
     * @param[in] c
     * @param[in] frac
     * @return Fraction<Z>
     */
    pub fn operator*(int&& c, const Fraction& frac) -> Fraction {
        return frac * Z(c);
    }

    /**
     * @brief
     *
     * @tparam _Stream
     * @tparam Z
     * @param[in] os
     * @param[in] frac
     * @return _Stream&
     */
    template <typename _Stream> pub fn operator<<(_Stream& os, const Fraction& frac)
        -> _Stream& {
        os << "(" << frac.num() << "/" << frac.den() << ")";
        return os;
    }

// For template deduction
// Integral{Z} Fraction(const Z &, const Z &) noexcept -> Fraction<Z>;

