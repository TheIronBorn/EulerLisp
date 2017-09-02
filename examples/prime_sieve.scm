(def ones (stream-cons 1 ones))
(stream-print 5 ones)
(puts "---")
(stream-print 5 (stream-map inc ones))
(puts "---")
(stream-print 5 (stream-add ones ones))
(puts "---")

(def fibs
  (stream-cons 0
    (stream-cons 1
      (stream-add fibs (stream-rst fibs)))))

(puts "---")
(stream-print 10 fibs)

(puts "---")
(def debug-stream
  (stream-map
    (fn (n) (do (print "Forcing ")
                (puts n)
                n))
    (natural-numbers-from 0)))

(defn is-multiple-of (div)
  (fn (n) (= 0 (% n div))))

(def multiples-of-five
  (stream-filter
    (is-multiple-of 5)
    debug-stream))

(puts (fst multiples-of-five))
(puts (fst (stream-rst multiples-of-five)))

(defn is-not-multiple-of (div)
  (fn (n) (!= 0 (% n div))))

(defn remove-multiples-of-first (stream)
  (stream-cons
    (fst stream)
    (remove-multiples-of-first (stream-filter
      (is-not-multiple-of (fst stream))
      (stream-rst stream)))))


(puts "---")
(stream-print 20 (remove-multiples-of-first (natural-numbers-from 2)))


(def primes (remove-multiples-of-first (natural-numbers-from 2)))
(puts "---")
(puts (stream-nth 200 primes))

; (stream-print 2 multiples-of-five)

; (stream-print 10 ones)

; (def twos (stream-add ones ones))

; (stream-print 10 twos)

; (def naturals
;      (stream-cons 1 (stream-add naturals ones)))

; (stream-print 10 naturals)

; (defn not-multiple (n)
;       (fn (x) (!= 0 (% x n))))

; (def odds
;      (stream-filter naturals (not-multiple 2)))

; (puts "odds")
; (stream-print 10 odds)

; (def fibs
;      (stream-cons
;        0
;        (stream-cons
;          1
;          (stream-add fibs (stream-rst fibs)))))

; (puts "fibs")
; (stream-print 10 fibs)

; (defn sieve (s)
;       (stream-cons
;         (fst s)
;         (sieve
;           (stream-filter
;             (stream-rst s)
;             (not-multiple (fst s))))))


; (def primes (sieve (stream-rst naturals)))

; (puts "primes")
; (stream-print 10 primes)

; (puts (stream-nth 50 primes))

; (puts "map")



