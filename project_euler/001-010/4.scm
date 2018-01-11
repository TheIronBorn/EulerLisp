; Solved: 17.12.17
; Changes:
;   * implement TCO for cond

(defn palindrome? (lst)
      (= lst (reverse lst)))

(def from 100)
(def to 999)

(defn solve (a b prod)
      (cond
        ((> a b) (solve from (inc b) prod))
        ((> b to) prod)
        ((and (> (* a b) prod) (palindrome? (digits (* a b))))
         (solve (inc a) b (* a b)))
        (else
         (solve (inc a) b prod))))

(println "Solution: " (solve from from 0))
