; Solved 8.1.2018

(defn calc-b (p a)
      (div (- (* 2 p a) (* p p))
           (* 2 (- a p))))

(defn has-solution? (p a)
  (let* ([b (calc-b p a)]
         [c (- p (+ a b))])
        (= (+ (* a a) (* b b))
           (* c c))))

(defn count-solutions (p)
  (~> (range~ 1 (div p 3))
      (count~ &(has-solution? p &1))))

; the first pair is 3 + 4 + 5 = 12
; and p must be even,
; otherwise a^2 + b^2 = c^2 is not possible
(~> (range 12 1000 2)
    (max-by count-solutions)
    solution)
