(let ((a 1) (b 5))
    (def when (macro (cond body)
        `(if ,cond
            (let () ,@body)
            nil)))
    
    (when (= 1 1) '(a (+ a b)))
)