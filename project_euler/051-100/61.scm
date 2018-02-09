; Solved 22.1.2018

(defn valid? (n) (not (zero? (% (div n 10) 10))))

(defn find-four-digit (f (n 1) (acc '()))
  (let ([next (f n)])
    (cond
      [(< next 1000) (find-four-digit f (inc n) acc)]
      [(> next 9999)
       (~> acc
           (select valid?)
           (map &(divmod &1 100)))]
      [else (find-four-digit f (inc n) (cons next acc))])))

; Start w/ octagonal because that has the fewest elements
(def numbers
     (map find-four-digit
          (list octagonal heptagonal hexagonal pentagonal square triangular)))

(defn find-sequences (start end lists (acc '()))
  (if (= 1 (length lists))
      (map list
        (select
          &(= &1 (cons start end))
          (fst lists)))
      (flatmap
        (fn (x)
            (map &(cons x &1)
              (find-sequences (rst x) end (rst lists))))
        (select
          &(= (fst &1) start)
          (fst lists)))))

(defn sequences (first rest)
     (flatmap
       (fn (x)
         (map &(cons x &1)
               (find-sequences (rst x) (fst x) rest)))
       first))

(~>
  (rst numbers)
  permutations
  (flatmap &(sequences (fst numbers) &1))
  fst
  (map &(+ (* 100 (fst &1)) (rst &1)))
  sum
  solution)
