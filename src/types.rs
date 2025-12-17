#[derive(Clone, Debug, Copy)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self {re, im }
    }

    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn mult(&self, other: &Complex) -> Complex {
        Complex {
        re: self.re * other.re - self.im * other.im,
        im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn scale(self, scalar: f64) -> Complex {
        Complex {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }

    pub fn conj(self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}



