; Solved: 18.1.18

(def numbers
  (~>
    (file-read "./project_euler/091-100/99.txt")
    lines
    (map &(map string->number (string-split "," &1)))))

(defn be> (be1 be2)
  (let (b1 (fst be1) e1 (frst be1)
        b2 (fst be2) e2 (frst be2))
    (> (* e1 (log b1 b2))
       e2)))

(defn solve (numbers line max-line max-be)
  (if (nil? numbers)
      max-line
      (if (be> (fst numbers) max-be)
          (solve (rst numbers) (inc line) line (fst numbers))
          (solve (rst numbers) (inc line) max-line max-be))))

(println (solve (rst numbers) 2 1 (fst numbers)))
