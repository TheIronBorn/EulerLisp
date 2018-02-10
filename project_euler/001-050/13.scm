; Solved: 29.12.2017

(def input
  (input-file-lines "project_euler/input-files/13.txt"))

(defn parse-number (x)
   (map char->digit
        (select char-numeric? (string->chars x))))

(def numbers (map &(reverse (parse-number &1)) input))

(defn cut (n)
  (if (> n 9999999999)
      (cut (div n 10))
      n))

; This makes sure the number doesn't get to big,
; note that at some point acc might be bigger than 10^10,
; but we can go up to 10^18 without overflows
; and just cut it to the right size later
(defn solve (numbers (acc 0) (p 0))
  (if (nil? (fst numbers))
      (cut acc)
      (let*
        ([cur-sum (sum (map fst numbers))]
         [next-sum (+ acc (* cur-sum (pow 10 p)))])
        (if (> next-sum 9999999999)
            (solve (map rst numbers) (div next-sum 10) p)
            (solve (map rst numbers) next-sum (inc p))))))

(solution (solve numbers))
