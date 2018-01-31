; Solved 8.1
;
; Brute force solution: 120s
; Improved to 1s
; using the fact that b can be caluclated
; instead of guessed

(defn calc-b (p a)
      (div (- (* 2 p a) (* p p))
           (* 2 (- a p))))

(defn has-solution? (p a)
  (let* (b (calc-b p a)
         c (- p (+ a b)))
        (= (+ (* a a) (* b b))
           (* c c))))

(defn count-solutions (p a solutions)
  (if (>= a (div p 3))
      solutions
      (if (has-solution? p a)
          (count-solutions p (inc a) (inc solutions))
          (count-solutions p (inc a) solutions))))

; the first pair is 3 + 4 + 5 = 12
; and p must be even,
; otherwise a^2 + b^2 = c^2 is not possible
(~>
  (range 12 1000 2)
  (max-by &(count-solutions &1 1 0))
  (println "Solution: "))
