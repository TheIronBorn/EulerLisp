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
      (if (> from to) acc
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

(defn select (pred arr)
  (cond
    ((nil? arr) '())
    ((pred (fst arr)) (cons (fst arr) (select pred (rst arr))))
    (else (select pred (rst arr)))))

(defn reject (pred arr)
  (cond
    ((nil? arr) '())
    ((pred (fst arr)) (reject pred (rst arr)))
    (else (cons (fst arr) (reject pred (rst arr))))))

(defn reduce (f acc arr)
  (if (nil? arr)
      acc
      (reduce f (f (fst arr) acc) (rst arr))))

(defn empty? (lst)
      (zero? (length lst)))

(defn any? (pred arr)
  (cond
    ((nil? arr) #f)
    ((pred (fst arr)) #t)
    (else (any? pred (rst arr)))))

(defn all? (pred arr)
  (cond
    ((nil? arr) #t)
    ((pred (fst arr)) (all? pred (rst arr)))
    (else #f)))

(defn zip lists
      (if (any? nil? lists)
          '()
          (cons (map fst lists)
                (apply zip (map rst lists)))))

(defn product (lst) (reduce * 1 lst))
(defn sum (lst) (reduce + 0 lst))

(defn take (n lst)
      (if (zero? n)
          '()
          (cons (fst lst) (take (dec n) (rst lst)))))

(defn chunks (size lst)
      (if (< (length lst) size)
          '()
          (cons (take size lst)
                (chunks size (rst lst)))))
