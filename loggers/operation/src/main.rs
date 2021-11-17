use log::info;
struct Complex {
    real: f64,
    img: f64,
}
fn main() {
env_logger::init();
    // Initializing complex_1
    let complex_1 = Complex {
        real: 2.0,
        img: 3.0,
    };
    // Initializing complex_2
    let complex_2 = Complex {
        real: 3.0,
        img: 2.0,
    };
    add(&complex_1, &complex_2);
    subtract(&complex_1, &complex_2);
    multiply(&complex_1, &complex_2);
}
/// Function 'add' adds two  complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
fn add(complex_1: &Complex, complex_2: &Complex) {
    let real_part = complex_1.real + complex_2.real;
    let imaginary_part = complex_1.img + complex_2.img;
   info!("Sum is: {} + {}i", real_part, imaginary_part);
}
/// Function 'subtract' subtracts two complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
fn subtract(complex_1: &Complex, complex_2: &Complex) {
    let real_part = complex_1.real - complex_2.real;
    let imaginary_part = complex_1.img - complex_2.img;
    info!("Difference is: {} + {}i", real_part, imaginary_part);
}
/// Function 'multiply' multiplies two complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
fn multiply(complex_1: &Complex, complex_2: &Complex) {
    let product_1 = complex_1.real * complex_2.real;
    let product_2 = complex_1.img * complex_2.img;
    let product_3 = (complex_1.real + complex_1.img) * (complex_2.real + complex_2.img);
    let real_part = product_1 - product_2;
    let imaginary_part = product_3 - (product_1 + product_2);

    info!("Product is: {} + {}i", real_part, imaginary_part);
}
