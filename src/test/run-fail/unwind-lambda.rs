// error-pattern:fail

fn main() {
    let cheese = "roquefort";
    let carrots = @"crunchy";

    fn (tasties: @str, macerate: block(str)) {
        macerate(*tasties);
    } (carrots, { |food|
        let mush = food + cheese;
        lambda() {
            let chew = mush + cheese;
            fail "so yummy"
        } ();
    });
}