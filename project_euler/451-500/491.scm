; Solved: 11.3.2018

;; A number is divisible by 11
;; if its alternating digital sum is divisible by 11.
;;
;; Create all possible distributions of the digits 0-9
;; (both in even positions, both in odd positions, one in each).
;; Digits that appear in both positions can be ingored
;; because their alternating sum is 0.

; 0 -> both digits in even positions
; 1 -> one digit even, one odd
; 2 -> both in odd positions
(defn number->choices (n)
  (defn inner (n remaining acc)
    (if (zero? remaining)
        acc
        (inner (div n 3)
               (dec remaining)
               (cons (% n 3) acc))))
  (inner n 10 '()))

; If none of the zeros is in even position
; (could be the first digit)
; the result is 10!.
; If one/both of them are,
; there are only 9/8 valid choices for the first digit
; (the non-zero digits).
(defn possibilities-even (c acc-e)
  (case (fst c)
        [0 (/ (*  8 (fac 9)) (pow 2 acc-e))]
        [1 (/ (*  9 (fac 9)) (pow 2 acc-e))]
        [2 (/ (* 10 (fac 9)) (pow 2 acc-e))]))

(defn possibilities-odd (c acc-o)
  (/ (fac 10) (pow 2 acc-o)))

; Check if the number of even-pos digits
; is the same as the number of odd-pos digits
;
; Accumulate all useful information
; * number of even/both/odd-pos digits
; * sum of digits in even positions
(defn preprocess (c-init)
  (defn inner (c d acc-e acc-o even-ds)
    (if (nil? c)
        (list c-init acc-e acc-o even-ds)
        (case (fst c)
          [0 (inner (rst c) (inc d) (inc acc-e) acc-o (+ even-ds d d))]
          [1 (inner (rst c) (inc d) acc-e  acc-o (+ even-ds d))]
          [2 (inner (rst c) (inc d) acc-e (inc acc-o) even-ds)])))
  (inner c-init 0 0 0 0))

(def limit (dec (pow 3 10)))

; Sum of all digits 0, 0, 1, 1, ..., 9, 9
(def full-ds (* 2 (gauss-sum 9)))

(~> (range-stream 0 limit)
    (stream-map &(preprocess (number->choices &1)))
    ; Check if the distribution of digits is valid
    ; #even = #odd and would yield something divisible by 11
    (stream-select (fn (s)
                     (and (= (frst s) (frrst s))
                          (divides? 11 {{2 * (frrrst s)} - 90}))))
    (stream-map
      (fn (s)
        (* (possibilities-even (fst s) (frst s))
           (possibilities-odd (fst s) (frrst s)))))
    stream-sum
    solution)
