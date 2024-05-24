use analyze_macro::analyze;

fn main() {
    println!(
        "{}",
        analyze!(
            /// outer comment
            /** comment block */
            struct Example {
                //! inner comment
                /*! inner comment block */
                val: String
            }
        )
    );
}
