(let ((a 1) (b 5))
    (def when
        (macro (cond body)
            (quasiquote
                (if (unquote cond)
                    (let () (splice-unquote body))
                    nil))))

    (macroexpand (quote (when (= 1 1) (quote (a (+ a b))))))
)