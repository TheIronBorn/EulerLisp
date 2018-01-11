; Solved: 17.12.17
; Changes:
;   * implement TCO for cond

(defn digits (n) (digits_ n '()))
(defn digits_ (n acc)
  (if (= n 0)
      acc
      (let ((digit (% n 10))
            (n (/ n 10)))
        (digits_ n (cons digit acc)))))

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
