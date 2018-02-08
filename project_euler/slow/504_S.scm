; Solved: 26.1.18
; Time: 16:41
; Solution: 694687

(defn on-line (a b)
  (dec (gcd a b)))

(defn in-triangle (a b)
    (div
      (- (* (dec a) (dec b))
         (on-line a b))
      2))

(def precalc
  (flatmap
    (fn (a) (map &(in-triangle a &1) (range 1 100)))
    (range 1 100)))

(defn lookup-in-triangle (a b)
      (list-ref precalc (+ (* 100 a) b)))

(defn inside (a b c d)
  (+ 
    (lookup-in-triangle a b)
    (lookup-in-triangle b c)
    (lookup-in-triangle c d)
    (lookup-in-triangle d a)
    a b c d
    1))

(defn square-inside? (a b c d)
      (square? (inside a b c d)))

; Work with 0 <= n < 100
; instead of 1 <= n <= 100
; to safe a few calls to `dec`
(~>
  (combinations~ 4 (range 0 (dec 100)))
  (count~ &(apply square-inside? &1))
  (println "Solution: "))
