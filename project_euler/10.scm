; Solved: 20.12.17
; Changes:

(defn is-not-multiple-of (div)
  (fn (n) (!= 0 (% n div))))

(defn remove-multiples-of-first (stream)
  (stream-cons
    (fst stream)
    (remove-multiples-of-first
      (stream-filter
        (is-not-multiple-of (fst stream))
        (stream-rst stream)))))
        
(def primes (remove-multiples-of-first (natural-numbers-from 2)))

(println (stream-nth 10000 primes))
