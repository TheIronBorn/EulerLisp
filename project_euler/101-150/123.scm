; Solved: 28.1.18
; TODO: Implement something like `.each_with_index` for streams

(defn large-enough? (pn)
  (let ([p (fst pn)] [n (rst pn)])
    (let ([mod (* p p)])
      (>
        (% (+ (modexp (dec p) n mod)
              (modexp (inc p) n mod))
           mod)
        10000000000))))

; A hacky way to add their index to elements of a stream
(defn make-counter ()
  (let ([c 1])
    (fn () (set! c (inc c)) c)))
(def cnt (make-counter))

(~>
  (step~ 3 2)
  (select~ prime?)
  (map~ &(cons &1 (cnt)))
  (select~ large-enough?)
  first~
  rst
  (println "Solution: "))
