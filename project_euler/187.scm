; Solved: 28.1.18

(def ps
     (~> (range~ 3 50000000 2)
         (select~ prime?)
         collect))

(def maxl (length ps))

; No need to check if i < maxl,
; there are primes > 10e8 / 3
; so for all primes the product will be > 10e8
; before reaching the end of the array
(defn loop (p1 i (acc 0))
  (let ([prod (* p1 (list-ref ps i))])
    (if (> prod 100000000)
      acc
      (loop p1 (inc i) (inc acc)))))

; Add all products 2 * other prime
; and 2 * 2
(~> (range~ 0 (dec maxl))
    (map~ &(loop (list-ref ps &1) &1))
    sum~
    (+ maxl 1)
    (println "Solution: "))
