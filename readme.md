# Sudoku Solver + Generator
Zbudowane za pomocą [Relm4](https://relm4.org).
Aplikacja może być nieczytelna w systemowym darkmode.

## Uruchamianie
Do uruchomienia aplikacji wymagane jest wiele bibliotek systemowych wymaganych przez relm4/gtk.

W związku z tym po wpisaniu `cargo run` (koniecznie w folderze zawierającym src/ , bo inaczej .css się nie załaduje) możliwe, że pojawią się błędy braku bibliotek: np.  
glib2.0-dev  
libgtk-4-dev

Niestety jest to zależne od komputera i nie jestem w stanie podać dokładnej listy :(.




## Początkowy opis projektu
Chciałbym stworzyć aplikację, która umożliwi użytkownikowi generowanie sudoku o wzbogaconych regułach.
Użytkownik wybiera reguły i wstawia część cyfr (może zero), a następnie program znajduje mu początkową planszę, która ma jednoznaczne rozwiązanie.

Przykładowe dodatkowe zasady:
- podzbiór pól sumuje się do zdefiniowanej przez twórcę wartości
- na przekątnych znaleźć ma się permutacja od 1 do n
- zamiast klasycznych kwadratów można zdefinować dowolne obszary o polu n, które mają zawierać permutację
- więcej pomysłów może pojawić się w trakcie

Dodatkowym feature aplikacji będą:
- możliwość rozwiązywania wygenerowanego sudoku przez użytkownika
- wyświetlanie "podpowiedzi", czyli wszystkich możliwych, niesprzecznych liczb do wpisania (coś jakby silnik rozwiązujący z głębokością 1)
- kolorowy interfejs graficzny
* możliwość zapisania wygenerowanego sudoku z customowymi zasadami na później

W generowaniu planszy można będzie pokusić się o jakąś współbieżność przy backtrackowaniu.
Ogólnie efektywność programu będzie ważnym elementem całej aplikacji, na razie założyłbym, że celuję w program generujący sudoko 6x6 z przyczyn obliczeniowych, jeśli się uda to chciałbym to doprowadzić do 9x9

Etap 1:
- (DONE) napisanie programu, który dla wejścia będącego standardową planszą sudoku znajduje jego rozwiązanie (rekurencyjnie i jednowątkowo)
- stworzenie interfejsu, który pozwala dodawać dodatkowe ograniczenia, przykładowo zbiory pól, które mają spełniać jeden z dwóch warunków (suma/bycie permutacją)
- (DONE) wzbogacenie programu rozwiązującego o sprawdzanie dodanych powyższych warunków
- powyższe rzeczy na razie będą działy się w konsoli i miały CLI 

Etap 2:
- dodanie interfejsu graficznego do rozwiązywania
- dodanie interfejsu graficznego do generowania 
- stworzenie formatu pliku, możliwość zapisywania/wgrywania swoich plansz (wcześniej użytkownik będzie musiał sobie przerysowywać)

## Realizacja
Dostępne jest GUI aplikacji, w którym można wykonać wszystkie wyżej wymienione możliwości:

### Tryb planowania
Domyślnie na początku rozrywki ustawiony jest tryb planowania. Można w nim dodawać własne reguły. Po wciśnięciu "Add rule: " pojawią się 3 opcje zasad:
- permutation - użytkownik wyklikuje N pól i deklaruje, że w tej grze ma znaleźć się na nich permutacja
- sum - po wpisaniu wybranej przez siebie sumy i wciśnięciu enter pojawi się reguła sumy. Wybrane przez użytkownika pola będą musiały w tej grze sumować się dokładnie do określonej wartości
- relation - użytkownik wyklikuje 2 pola (kolejność jest ważna) i deklaruje, że wartość drugiego z nich ma być ściśle większa niż pierwszego.

Zasady można modyfikować w trakcie gry. Wystarczy wcisnąć wybraną przez siebie zasadę, aby ją edytować. Ponowne wciśnięcie pola usunie je z pól objętych zasadą.

W trybie gry wciśnięcie przycisku z regułą podświetli pola, których reguła dotyczy, aby pomóc w rozwiązywaniu.

## Tryb gry
Użytkownik zaczyna z pustą planszą.  
Można wprowadzić własne cyfry i rozwiązywać sudoku.   

Aby wprowadzić cyfrę, trzeba wcisnąć klawisz z tą cyfrą i kliknąć na pole, do którego chcemy ją wprowadzić. Alternatywnie można też poruszać się po planszy strzałkami i potwierdzać enterem.

W dowolnym momencie wciśnięcie 'g' wygeneruje planszę do rozwiązania (jeśli zasady są skomplikowane, to generator na planszy 9x9 może działać długo).  

Wciśnięcie klawisza 'p' pokaże podpowiedzi: dla każdego pola pojawią się wszystkie możliwe wartości, które mogą się w danym momencie na nim znaleźć. 
Ponowne kliknięcie klawisza 'p' usunie podpowiedzi.

W dowolnym momencie w trybie użytkownika można wcisnąć klawisz 'f'. Wbudowany solver rozwiąże planszę do końca lub zgłosi, że jest to niemożliwe. (ponownie, może być to ciężkie obliczeniowo).

Solver był testowany empirycznie i już się nie zapętla, więc na pewno się zatrzyma, ale może trwać to bardzo długo. 

Wciśnięcie klawisza 'v' wypisuje po prawej stronie ekranu wszystkie aktualnie niespełnione reguły.

Wciśnięcie klawisza 'h' odpala popup z skrótem powyższych komend.

## Zapis/odczyt z pliku
Na dole okna są dwa pola tekstowe. Można przy ich pomocy zapisać lub wczytać grę z pliku. W przypadku błędu program wypisze stosowny komunikat.

Wiktor Rutecki