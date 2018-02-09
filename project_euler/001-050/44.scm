; Solved: 13.1.2018

(defn find-first (a b)
     (if (>= b a)
       (find-first (inc a) 1)
       (let ([pa (pentagonal a)]
             [pb (pentagonal b)])
         (if (and (pentagonal? (+ pa pb)) (pentagonal? (- pa pb)))
             (- pa pb)
             (find-first a (inc b))))))

; To be perfectly sure this is the right solution,
; we could prove that P_n grows in such a way,
; that we would return the solution with the smallest difference first
(solution (find-first 2 1))
