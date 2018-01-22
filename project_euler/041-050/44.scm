; Solved: 13.1.18

(defn pentagonal (n)
      (div (* n (dec (* 3 n))) 2))

; P_n = (n * (3n - 1)) / 2
;     = (3n^2 - n) / 2
;
; x is pentagonal
; <=> x = (3n^2 - n) / 2 has an integer solution
; <=> 0 = 3n^2 - n - 2x
; 
; Quadratic formula:
;
; a = 3, b = -1, c = -2x
; det = b^2 - 4ac
;     = 1 + 24x
;
; x_1,2 = (-b +- sqrt(det)) / (2a)
;       = (1 +- sqrt(det)) / 6
;
; We only care about positive solutions,
; so we can ignore 1 - sqrt(det).
;
; => x is pentagonal,
;    if det is a square
;    and 1 + sqrt(det) is divisible by 6
(defn pentagonal? (n)
  (let* (det (inc (* 24 n))
         root (isqrt det))
    (and (= det (* root root))
         (divides? 6 (inc root)))))

(defn find-first (a b)
     (if (>= b a)
       (find-first (inc a) 1)
       (let (pa (pentagonal a)
             pb (pentagonal b))
         (if (and
               (pentagonal? (+ pa pb))
               (pentagonal? (- pa pb))
               )
           (- pa pb)
           (find-first a (inc b))))))

; To be perfectly sure this is the right solution,
; we could prove that P_n grows in such a way,
; that we would return the solution with the smallest difference first
(println "Solution: " (find-first 2 1))
