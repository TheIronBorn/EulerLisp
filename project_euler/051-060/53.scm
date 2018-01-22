; Solved 14.1

(defn fac-from (from to)
  (product~ (range~ from to)))

(defn choose (n r)
  (let (other (max r (- n r)))
    (div (fac-from (+ other 1) n)
         (fac-from 1 (- n other)))))

; Iterate over r
; and then r <= n <= 100
;
; By using (n+1 r) = (n r) (n r-1)
; we can quit early
; and avoid overflows
(defn solve (r n (greater 0))
  (cond
    (> r 100) greater
    (> n 100) (solve (inc r) (inc r) greater)
    else
    (if (> (choose n r) 1000000)
        (solve (inc r) (inc r)
               (+ greater (- 100 (- n 1))))
        (solve r (inc n) greater))))

(println "Solution: " (solve 1 1))
