; Solved: 22.12.17
; Deterministic miller-rabin

(defn witnesses (n)
  (cond
    (< n 2047) (list 2)
    (< n 1373653) (list 2 3)
    (< n 9080191) (list 31 73)
    (< n 25326001) (list 2 3 5)
    (< n 3215031751) (list 2 3 5 7)
    (< n 4759123141) (list 2 7 61),
    (< n 1122004669633) (list 2 13 23 1662803)
))

(defn mymodexp (base ex mod) (mymodexp_ base ex mod 1))
(defn mymodexp_ (base ex mod acc)
  (cond
    (zero? ex) acc
    (even? ex) (mymodexp_ (% (* base base) mod) (div ex 2) mod acc)
    else (mymodexp_ base (dec ex) mod (% (* acc base) mod))))

; Write n as 2^r * d, d odd
(defn factor2 (n) (factor2_ n 0))
(defn factor2_ (n r)
  (if (even? n)
      (factor2_ (>> n 1) (inc r))
      (cons r n)))

; returns true if probably prime, false if not
(defn witness-loop (n r d witnesses)
      (if (nil? witnesses)
          #t
          (let* (a (fst witnesses) 
                 x (mymodexp a d n))
            (if (or (= x 1) (= x (dec n)))
                (witness-loop n r d (rst witnesses))
                (if (subloop (dec r) x n)
                    (witness-loop n r d (rst witnesses))
                    #f)))))

(defn myprime? (n)
  (if (= n 2)
      #t
      (let (rd (factor2 (dec n)))
           (witness-loop n (fst rd) (rst rd)
                         (witnesses n)))))


; return true to continue, false to return 'composite'
(defn subloop (times x n)
      (if (zero? times)
          #f
          (let (x (% (* x x) n))
               (cond
                 (= x 1) #f
                 (= x (dec n)) #t
                 else (subloop (dec times) x n)))))

(defn nth-prime (n) (nth-prime_ (- n 3) 5))
(defn nth-prime_ (n cur)
      (println n)
      (if (myprime? cur)
          (do
            (if (zero? n)
                cur
                (nth-prime_ (dec n) (+ cur 2)))
          )
          (nth-prime_ n (+ cur 2))))


(println (nth-prime 10001))
