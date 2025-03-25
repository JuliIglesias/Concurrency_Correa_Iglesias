use http_server::leibniz_approximation;

#[test]
fn first_term() {
    let (approx, _) = leibniz_approximation(0);
    assert_eq!(4.0, approx);
}
