; Solved: 

(defn streak (n (s 1) (acc 0))
  (if (divides? s n)
      (streak (inc n) (inc s) (inc acc))
      acc))

(defn p2 (n)
  (~>
    (range~ 1 n)
    (select~ &(= 0 (% (+ &1 1) 2)))
    (count~ &(!= 0 (% (+ &1 2) 3)))))

(defn p11 (n)
  (~>
    (range~ 1 n)
    (select~ &(= 0 (% (+ &1 1) 2)))
    (select~ &(= 0 (% (+ &1 2) 3)))
    (select~ &(= 0 (% (+ &1 3) 4)))
    (select~ &(= 0 (% (+ &1 4) 5)))
    (select~ &(= 0 (% (+ &1 5) 6)))
    (select~ &(= 0 (% (+ &1 6) 7)))
    (select~ &(= 0 (% (+ &1 7) 8)))
    (select~ &(= 0 (% (+ &1 8) 9)))
    (select~ &(= 0 (% (+ &1 9) 10)))
    (select~ &(= 0 (% (+ &1 10) 11)))
    (count~ &(!= 0 (% (+ &1 11) 12)))
    ))

(defn p (s n)
  (~>
    (range~ 3 (dec n) 2)
    (count~ &(= (streak &1) s))))

(defn p1 (n) n)

; (println (p1 (pow 4 1)))
; (println (p2 (pow 4 2)))
; (println (p 3 (pow 4 3)))
; (println (p 4 (pow 4 4)))
; (println (p 5 (pow 4 5)))
; (println (p 6 (pow 4 6)))
; (println (p 7 (pow 4 7)))
; (println (p 8 (pow 4 8)))
; (println (p 9 (pow 4 9)))
; (println (p 10 (pow 4 10)))
; (println (p 11 (pow 4 11)))
(println (p11 (pow 4 11)))
