(defsyntax delay () (
  ((delay expression) (make-promise (fn () expression)))))

(defn make-promise (proc)
  (let ([result-ready? #f]
        [result #f])
    (fn ()
      (if result-ready?
          result
          (let ([x (proc)])
            (if result-ready?
                result
                (do (set! result-ready? #t)
                    (set! result x)
                    result)))))))

(defn force (obj) (obj))

(defsyntax stream-cons () (
  ((stream-cons head tail) (cons head (delay tail)))))

(defn step-stream (from by)
  (stream-cons from (step-stream (+ from by) by)))

(defn stream-rst (stream)
  (force (rst stream)))

(defn stream-select (pred stream)
  (cond
    [(nil? stream) '()]
    [(pred (fst stream))
     (stream-cons
       (fst stream)
       (stream-select pred (stream-rst stream)))]
    [else
     (stream-select pred (stream-rst stream))]))

(defn stream-nth (n stream)
  (cond
    [(nil? stream) '()]
    [(zero? n) (fst stream)]
    [else
     (stream-nth (dec n) (stream-rst stream))]))

(defn range-stream (from to . by)
  (defn inner (from to by)
    (if {from > to}
        '()
        (stream-cons
          from
          (inner {from + by} to by))))
  (if (nil? by)
      (inner from to 1)
      (inner from to (fst by))))

(defn stream-map (fun stream)
  (if (nil? stream)
      '()
      (stream-cons
        (fun (fst stream))
        (stream-map fun (stream-rst stream)))))

(defn stream-collect (stream)
  (defn inner (stream acc)
    (if (nil? stream)
        (reverse acc)
        (inner
          (stream-rst stream)
          (cons (fst stream) acc))))
  (inner stream '()))

(defn stream-flatmap (fun source-stream)
  (defn inner (current fun source-stream)
    (if (nil? current)
        (if (nil? source-stream)
            '()
            (inner (fun (fst source-stream))
                   fun
                   (stream-rst source-stream)))
        (stream-cons
          (fst current)
          (inner
            (stream-rst current)
            fun
            source-stream))))
  (if (nil? source-stream)
      '()
      (inner '() fun source-stream)))

(defn stream-take-while (pred stream)
  (cond
    [(nil? stream) '()]
    [(pred (fst stream))
     (stream-cons
       (fst stream)
       (stream-take-while
         pred
         (stream-rst stream)))]
    [else '()]))

(defn stream-reduce (fun acc stream)
  (if (nil? stream)
      acc
      (stream-reduce
        fun
        (fun (fst stream) acc)
        (stream-rst stream))))

(defn stream-sum (stream)
  (stream-reduce + 0 stream))

(defn stream-product (stream)
  (stream-reduce * 1 stream))

(defn primes-stream (capacity)
  ;; Bitvectors are initialized as false,
  ;; to save time, composite numbers are marked as true
  ;; and primes as false.
  ;;
  ;; Tricks to improve performance:
  ;;
  ;; 1. Only store odd numbers
  ;; 2. Make alternating steps of 2 and 4
  ;;    to skip multiples of 3 `(bitwise-xor step 3)`.
  ;;    this is only valid for primes > 5,
  ;;    so the 2, 3 and 5 must be included manually
  (def cap2 (div capacity 2))
  (def sieve (make-bitvector cap2))
  (defn remove-multiples (n multiple)
    (when {multiple <= capacity}
      (bitvector-set! sieve (div multiple 2))
      (remove-multiples n {multiple + n + n})))
  (defn init-sieve (cur)
    (if {cur <= capacity}
      (if (bitvector-get sieve (div cur 2))
          (init-sieve (+ 2 cur))
          (do
            (remove-multiples cur {3 * cur})
            (init-sieve (+ 2 cur))))))
  (init-sieve 3)
  (defn inner (cur step)
    (if {cur >= cap2}
        '()
        (if (bitvector-get sieve cur)
            (inner {cur + step} (bitwise-xor step 3))
            (stream-cons
              {{2 * cur} + 1}
              (inner {cur + step} (bitwise-xor step 3))))))
  (stream-cons 2
    (stream-cons 3
      (stream-cons 5
         (inner 3 2)))))
