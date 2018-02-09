; Solved: 3.2.2018

(defn count-solutions (n outer inner)
      (div (- outer inner) 2))

(defn make-even (n) (if (even? n) n (inc n)))
(defn make-odd (n) (if (odd? n) n (inc n)))

(defn min-inner (n outer)
  (let ([diff (- (square outer) n)])
    (if (> diff 0) (ceil (sqrt diff)) 1)))

(defn find-inner (n outer)
  (if (even? outer)
    (count-solutions n outer (make-even (min-inner n outer)))
    (count-solutions n outer (make-odd (min-inner n outer)))))

(defn max-outer (n) (inc (div n 4)))

(def n 1_000_000)

(~>
  (range~ 3 (max-outer n))
  (map~ &(find-inner n &1))
  sum~
  solution)
