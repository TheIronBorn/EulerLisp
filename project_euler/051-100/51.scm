; Solved: 20.1.2018

;; Which types of replacements could lead to
;; eight element prime families?
;;
;; A number is divisible by 3,
;; iff the sum of its digits is divisible by 3.
;;
;; This means replacing one digit
;; can never create a family of eight primes,
;; because at least 3 of the numbers will be divisible by 3.
;;
;; The same is true for two and four digits,
;; because $2n \mod 3 = 0, 2, 1, 0, 2, 1, \ldots$
;; and $4n \mod 3 = 0, 1, 2, 0, 1, 2, \ldots$.


; TODO: for p = 1123444
; only use 1123*** as replacement

(defn replace (n i d)
  (let ([p1 (pow 10 i)])
       (+ (% n p1)
          (- n (% n (* 10 p1)))
          (* d p1))))

; It can't be the last digit,
; otherwise some of the numbers in the 8-number family
; would be even
(defn replacements (last len)
  (if (= len 1)
      (map list (range 1 (dec last)))
      (flatmap
        (fn (x)
            (map
              &(cons x &1)
              (replacements x (dec len))))
        (range len (dec last)))))

(defn all-replacements (digits)
  (flatmap
    &(replacements digits &1)
    (range 1 (dec digits))))

(defn possible? (rep)
  (case (length rep)
    [1 #f]
    [2 #f]
    [3 #t]
    [4 #f]
    [5 #t]
    [6 #t]
    [7 #t]))

(def replacements5 (select possible? (all-replacements 5)))
(def replacements6 (select possible? (all-replacements 6)))
(def replacements7 (select possible? (all-replacements 7)))

(defn get-all-replacements (n)
  (case (inc (floor (log10 n)))
        [5 replacements5]
        [6 replacements6]
        [7 replacements7]))

(defn apply-replacement (n replacement digit)
  (if (nil? replacement)
      n
      (apply-replacement
        (replace n (fst replacement) digit)
        (rst replacement)
        digit)))

; The (> x n) is needed to avoid including
; 109 in the family of 111109
(def all-digits (range 0 9))
(defn maybe-creates-family (n replacement)
    (let* ([family (map &(apply-replacement n replacement &1) all-digits)]
           [family-primes (select &(and (prime? &1) {n <= &1}) family)])
      (if (= 8 (length family-primes))
          (list-min family-primes)
          #f)))

(defn maybe-family (n)
  (defn inner (reps)
    (cond
      [(nil? reps) '()]
      [(maybe-creates-family n (fst reps))]
      [else (inner (rst reps))]))
  (inner (get-all-replacements n)))

(defn solve (cur)
  (cond
    [(not (prime? cur)) (solve {cur + 2})]
    [(maybe-family cur)]
    [else (solve {cur + 2})]))

(solution (solve 56003))
