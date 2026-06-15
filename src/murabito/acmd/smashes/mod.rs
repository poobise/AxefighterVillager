mod attacks4; //fsmash
mod attackhi4;
mod attacklw4;

pub fn install() {
    attacks4::install();
    attackhi4::install();
    attacklw4::install();
}