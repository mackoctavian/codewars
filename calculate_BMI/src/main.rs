//Write function bmi that calculates body mass index (bmi = weight / height2).
fn main() {
    bmi(60, 30.0);
}

fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height * height);
    match bmi {
        bmi if bmi <= 18.5 => "Underweight",
        bmi if bmi <= 25.0 => "Normal",
        bmi if bmi <= 30.0 => "Overweight",
        _ => "Obese",
    }
}
