(let ((a 1) (b 2))
    (def avg (lambda (x y z) (/ (+ x y z) 3)))

    (let ((c 5))
        (avg a b c)
    )
)