fn main () {
	let p:f64 = 510_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//amount
	let a= p *(1.0+(r/100.0).powf(n));
	println!("Amount is{}",a );
	//compound interest
	let ci=a-p;
	println!("Compound interest is{}", ci);
}