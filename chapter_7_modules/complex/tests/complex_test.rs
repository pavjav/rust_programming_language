

#[cfg(test)]
mod tests {
    use complex::complex;
    #[test]
    fn initialize() {
        let _z_f32 = complex::Complex::new(2.0f32, 2.0f32);
        let _z_f64 = complex::Complex::new(2.0f64, 2.0f64);
        let _z_i32 = complex::Complex::new(2i32, 2i32);
        let _z_i64 = complex::Complex::new(2i64, 2i64);
    }
    #[test]
    fn arithmetic() {
        let z_f32 = complex::Complex::new(2.0f32, 2.0f32);
        let z_f64 = complex::Complex::new(2.0f64, 2.0f64);
        let z_i32 = complex::Complex::new(2i32, 2i32);
        let z_i64 = complex::Complex::new(2i64, 2i64);

        let w_f32 = complex::Complex::new(2.0f32, 2.0f32);
        let w_f64 = complex::Complex::new(2.0f64, 2.0f64);
        let w_i32 = complex::Complex::new(2i32, 2i32);
        let w_i64 = complex::Complex::new(2i64, 2i64);

        let _res = z_f32*&w_f32;
        let _res = &z_f32*&w_f32;
        let _res = &z_f32*w_f32;

        assert_eq!(z_f32*w_f32,complex::Complex::new(0f32,8f32));
        assert_eq!(z_f64*w_f64,complex::Complex::new(0f64,8f64));
        assert_eq!(z_i32*w_i32,complex::Complex::new(0i32,8i32));
        assert_eq!(z_i64*w_i64,complex::Complex::new(0i64,8i64));

        assert_eq!(z_f32+w_f32,complex::Complex::new(4f32,4f32));
        assert_eq!(z_f64+w_f64,complex::Complex::new(4f64,4f64));
        assert_eq!(z_i32+w_i32,complex::Complex::new(4i32,4i32));
        assert_eq!(z_i64+w_i64,complex::Complex::new(4i64,4i64));

        assert_eq!(z_f32-w_f32,complex::Complex::new(0f32,0f32));
        assert_eq!(z_f64-w_f64,complex::Complex::new(0f64,0f64));
        assert_eq!(z_i32-w_i32,complex::Complex::new(0i32,0i32));
        assert_eq!(z_i64-w_i64,complex::Complex::new(0i64,0i64));

        assert_eq!(-w_f32,complex::Complex::new(-2f32,-2f32));
        assert_eq!(-w_f64,complex::Complex::new(-2f64,-2f64));
        assert_eq!(-w_i32,complex::Complex::new(-2i32,-2i32));
        assert_eq!(-w_i64,complex::Complex::new(-2i64,-2i64));
        
        assert_eq!(w_f32.inverse(),complex::Complex::new(0.25f32,-0.25f32));
        assert_eq!(w_f64.inverse(),complex::Complex::new(0.25f64,-0.25f64));
        assert_eq!(w_i32.inverse(),complex::Complex::new(0i32,0i32));
        assert_eq!(w_i64.inverse(),complex::Complex::new(0i64,0i64));

        assert_eq!(w_f32.conj(),complex::Complex::new(2f32,-2f32));
        assert_eq!(w_f64.conj(),complex::Complex::new(2f64,-2f64));
        assert_eq!(w_i32.conj(),complex::Complex::new(2i32,-2i32));
        assert_eq!(w_i64.conj(),complex::Complex::new(2i64,-2i64));

        assert_eq!(w_f32.dot(z_f32),complex::Complex::new(8f32,0f32));
        assert_eq!(w_f64.dot(z_f64),complex::Complex::new(8f64,0f64));
        assert_eq!(w_i32.dot(z_i32),complex::Complex::new(8i32,0i32));
        assert_eq!(w_i64.dot(z_i64),complex::Complex::new(8i64,0i64));

        assert_eq!(w_f32.conj()*z_f32,complex::Complex::new(8f32,0f32));
        assert_eq!(w_f64.conj()*z_f64,complex::Complex::new(8f64,0f64));
        assert_eq!(w_i32.conj()*z_i32,complex::Complex::new(8i32,0i32));
        assert_eq!(w_i64.conj()*z_i64,complex::Complex::new(8i64,0i64));

        assert_eq!(w_f32.modulus(),8f32);
        assert_eq!(w_f64.modulus(),8f64);
        assert_eq!(w_i32.modulus(),8i32);
        assert_eq!(w_i64.modulus(),8i64);

    }
}
