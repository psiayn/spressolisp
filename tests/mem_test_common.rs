pub const MANY_CLOSURES: &str = "
(
    define a (
        lambda x (
            (((
                lambda x (
                    lambda x (
                        lambda x (
                            define aaa 100
                        )
                    )
                )
            )
            a) a) a
        )
    )
)
(define i 0)
(loop (< i 100) (define i (+ i 1)) (a 10))
";
