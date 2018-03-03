; Solved: 29.12.2017

(defn parse-number (x)
   (map char->digit
        (select char-numeric? (string->chars x))))

; This makes sure the number doesn't get to big,
; note that at some point acc might be bigger than 10^10,
; but we can go up to 10^18 without overflows
; and just cut it to the right size later
(defn solve (numbers)
  (defn inner (numbers acc p)
    (if (nil? (fst numbers))
        acc
        (let*
          ([cur-sum (reduce-sum fst numbers)]
           [next-sum (+ acc (* cur-sum (pow 10 p)))])
          (if (> next-sum 9999999999)
              (inner (map rst numbers) (div next-sum 10) p)
              (inner (map rst numbers) next-sum (inc p))))))
  (inner numbers 0 0))

;; `string->number` doesn't work here
;; because it can't parse bignums
(~> "project_euler/input-files/13.txt"
    input-file-lines
    (map &(reverse (parse-number &1)))
    solve
    number->digits
    (take 10)
    digits->number
    solution)
