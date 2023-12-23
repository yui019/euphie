(let ()
    (def f (lambda (a &optional b c &key d e &rest z) `(,a ,b ,c ,d ,e ,z)))
    (f 1 2 3 :d 4 5 :e '(54 65))
)