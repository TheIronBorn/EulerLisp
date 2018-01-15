; Solved 14.1

(defn choose (n r)
  (let (other (max r (- n r)))
    (/ (fac-from (+ other 1) n)
       (fac (- n other)))
      ))

(defn fac-from (from to (acc 1))
    (if (> from to)
        acc
        (fac-from (inc from) to (* acc from))))

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
    (let (res (choose n r))
      (if (> res 1000000)
          (solve
            (inc r)
            (inc r)
            (+ greater (- 100 (- n 1))))
          (solve
            r
            (inc n)
            greater)))))

(println "Solution: " (solve 1 1))
