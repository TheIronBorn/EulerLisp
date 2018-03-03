; Solved: 13.2.2018

(def numerals '(
  (1000 . "M") (900 . "CM")
  (500 . "D") (400 . "CD")
  (100 . "C") (90 . "XC")
  (50 . "L") (40 . "XL")
  (10 . "X") (9 . "IX")
  (5 . "V") (4 . "IV")
  (1 . "I")))

(defn find-numeral (n nums)
  (if {(ffst nums) <= n}
      (fst nums)
      (find-numeral n (rst nums))))

(defn format (n)
  (defn inner (n acc)
    (if {n >= 1}
      (let ([num (find-numeral n numerals)])
        (inner
          {n - (fst num)}
          (cons (rst num) acc)))
      (apply str (reverse acc))))
  (inner n '()))

(defn maybe-frst (p)
  (if (nil? (rst p))
      '()
      (frst p)))

(defn parse (n)
  (defn parse_ (cs acc)
    (if (nil? cs)
        acc
        (let ([cur (fst cs)])
          (cond
            [(= cur #\M) (parse_ (rst cs) {acc + 1000})]
            [(= cur #\D) (parse_ (rst cs) {acc + 500})]
            [(= cur #\C)
             (case (maybe-frst cs)
               [#\M (parse_ (rrst cs) {acc + 900})]
               [#\D (parse_ (rrst cs) {acc + 400})]
               [else (parse_ (rst cs) {acc + 100})])]
            [(= cur #\L) (parse_ (rst cs) {acc + 50})]
            [(= cur #\X)
             (case (maybe-frst cs)
               [#\C (parse_ (rrst cs) {acc + 90})]
               [#\L (parse_ (rrst cs) {acc + 40})]
               [else (parse_ (rst cs) {acc + 10})])]
            [(= cur #\V) (parse_ (rst cs) {acc + 5})]
            [(= cur #\I)
             (case (maybe-frst cs)
               [#\X (parse_ (rrst cs) {acc + 9})]
               [#\V (parse_ (rrst cs) {acc + 4})]
               [else (parse_ (rst cs) {acc + 1})])])))) 
  (parse_ (string->chars n) 0))


(defn saved (num)
  {(string-length num) - (~> num parse format string-length)})

(~> "./project_euler/input-files/89.txt"
    input-file-lines
    (reduce-sum saved)
    solution)
