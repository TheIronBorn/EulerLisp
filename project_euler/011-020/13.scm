; Solved: 29.12.17

(def input
     (~> "project_euler/011-020/13.txt"
         file-read
         lines
         (reject empty?)))

(defn parse-byte (b) (- b 48))
(defn is-number? (b) (and (>= b 48) (<= b 57)))

(defn parse-number (x)
   (map parse-byte
        (select is-number? (string-bytes x))))

(def numbers (map (fn (x) (reverse (parse-number x))) input))

(defn cut (n)
  (if (> n 9999999999)
      (cut (/ n 10))
      n))

; This makes sure the number doesn't get to big,
; note that at some point acc might be bigger than 10^10,
; but we can go up to 10^18 without overflows
; and just cut it to the right size later
(defn solve (numbers acc p)
  (if (nil? (fst numbers))
      (cut acc)
      (let*
        ((cur-sum (sum (map fst numbers)))
         (next-sum (+ acc (* cur-sum (pow 10 p)))))
        (if (> next-sum 9999999999)
            (solve (map rst numbers) (/ next-sum 10) p)
            (solve (map rst numbers) next-sum (inc p))))))

(println "Solution: " (solve numbers 0 0))
