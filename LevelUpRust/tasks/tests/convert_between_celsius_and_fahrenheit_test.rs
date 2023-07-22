use tasks::convert_between_celsius_and_fahrenheit::*;

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0, Scale::Celsius);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Temperature::new(100.0, Scale::Celsius);
    assert!((hot.to_fahrenheit() - 212.0) < 0.01);
    assert!((hot.to_fahrenheit() - 212.0) >= 0.0);
}

#[test]
fn freezing() {
    let freezing = Temperature {
        degrees: Temperature::new(0.0, Scale::Celsius).to_fahrenheit(),
        scale: Scale::Fahrenheit,
    };

    assert!(freezing.to_celsius() < 0.001);
    assert!(freezing.to_celsius() > -0.01);
}
