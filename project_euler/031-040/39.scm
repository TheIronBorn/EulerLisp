; Solved 8.1
;
; Brute force solution: 120s
; Improved to 1s
; using the fact that b can be caluclated
; instead of guessed

(defn calc-b (p a)
      (/ (- (* 2 p a) (* p p))
         (* 2 (- a p))))

(defn has-solution? (p a)
  (let* (b (calc-b p a)
         c (- p (+ a b)))
        (= (+ (* a a) (* b b))
           (* c c))))

(defn count-solutions (p a solutions)
  (if (>= a (/ p 3))
      solutions
      (if (has-solution? p a)
          (count-solutions p (inc a) (inc solutions))
          (count-solutions p (inc a) solutions))))

(defn solve (p max-p max-solutions)
  (if (> p 1000)
      (cons max-p max-solutions)
      (let (solutions (count-solutions p 1 0))
        (if (> solutions max-solutions)
            (solve (+ p 2) p solutions)
            (solve (+ p 2) max-p max-solutions)))))

; 2 wont work, because a + b + c >= 3
; and p must be even,
; otherwise a^2 + b^2 = c^2 is not possible
(println "Solution: " (solve 12 12 0))
