(defn ffst (lst) (fst (fst lst)))
(defn frst (lst) (fst (rst lst)))
(defn rfst (lst) (rst (fst lst)))
(defn rrst (lst) (rst (rst lst)))

(defn fffst (lst) (fst (fst (fst lst))))
(defn ffrst (lst) (fst (fst (rst lst))))
(defn frfst (lst) (fst (rst (fst lst))))
(defn frrst (lst) (fst (rst (rst lst))))
(defn rffst (lst) (rst (fst (fst lst))))
(defn rfrst (lst) (rst (fst (rst lst))))
(defn rrfst (lst) (rst (rst (fst lst))))
(defn rrrst (lst) (rst (rst (rst lst))))

(defn ffffst (lst) (fst (fst (fst (fst lst)))))
(defn fffrst (lst) (fst (fst (fst (rst lst)))))
(defn ffrfst (lst) (fst (fst (rst (fst lst)))))
(defn ffrrst (lst) (fst (fst (rst (rst lst)))))
(defn frffst (lst) (fst (rst (fst (fst lst)))))
(defn frfrst (lst) (fst (rst (fst (rst lst)))))
(defn frrfst (lst) (fst (rst (rst (fst lst)))))
(defn frrrst (lst) (fst (rst (rst (rst lst)))))
(defn rfffst (lst) (rst (fst (fst (fst lst)))))
(defn rffrst (lst) (rst (fst (fst (rst lst)))))
(defn rfrfst (lst) (rst (fst (rst (fst lst)))))
(defn rfrrst (lst) (rst (fst (rst (rst lst)))))
(defn rrffst (lst) (rst (rst (fst (fst lst)))))
(defn rrfrst (lst) (rst (rst (fst (rst lst)))))
(defn rrrfst (lst) (rst (rst (rst (fst lst)))))
(defn rrrrst (lst) (rst (rst (rst (rst lst)))))

(defn range (from to)
      (range_ from to '()))
(defn range_ (from to acc)
      (if (< to from) acc
          (range_ from (dec to) (cons to acc))))

(defn flatmap (f arr)
      (if (nil? arr)
          '()
          (append (f (fst arr))
                (flatmap f (rst arr)))))

(defn delete-nth (n lst)
  (if (= n 0)
      (rst lst)
      (cons (fst lst)
            (delete-nth (dec n) (rst lst)))))

(defn select (pred arr (acc '()))
  (cond
    (nil? arr) acc
    (pred (fst arr)) (select pred (rst arr)
                             (push acc (fst arr)))
    else (select pred (rst arr) acc)))

(defn reject (pred arr (acc '()))
  (cond
    (nil? arr) acc
    (pred (fst arr)) (reject pred (rst arr) acc)
    else (reject pred (rst arr)
                 (push acc (fst arr)))))

(defn reduce (f acc arr)
  (if (nil? arr)
      acc
      (reduce f (f (fst arr) acc) (rst arr))))

(defn count (pred arr (acc 0))
  (cond
    (nil? arr) acc
    (pred (fst arr)) (count pred (rst arr) (inc acc))
    else (count pred (rst arr) acc)))

(defn reduce-sum (f arr)
  (reduce (fn (x acc)
              (+ acc (f x)))
          0
          arr))

(defn reduce-product (f arr)
  (reduce (fn (x acc)
              (* acc (f x)))
          1
          arr))

(defn any? (pred arr)
  (cond
    (nil? arr) #f
    (pred (fst arr)) #t
    else (any? pred (rst arr))))

(defn all? (pred arr)
  (cond
    (nil? arr) #t
    (pred (fst arr)) (all? pred (rst arr))
    else #f))

(defn zip (. lists)
      (if (any? nil? lists)
          '()
          (cons (map fst lists)
                (apply zip (map rst lists)))))

(defn product (lst) (reduce * 1 lst))
(defn sum (lst) (reduce + 0 lst))

(defn product~ (lst) (reduce~ * 1 lst))
(defn sum~ (lst) (reduce~ + 0 lst))

(defn take (n lst)
      (if (or (zero? n) (nil? lst))
          '()
          (cons (fst lst) (take (dec n) (rst lst)))))

(defn chunks (size lst (acc '()))
      (if (< (length lst) size)
          acc
          (chunks size
                  (rst lst)
                  (push acc (take size lst)))))

(defn uniq (arr (last -1) (acc '()))
  (cond
    (nil? arr) acc
    (= last (fst arr)) (uniq (rst arr) last acc)
    else (uniq (rst arr) (fst arr) (cons (fst arr) acc))))

