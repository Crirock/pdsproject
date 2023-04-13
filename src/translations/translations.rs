use iced::widget::Text;

use crate::translations::types::language::Language;

pub fn choose_adapters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
        Language::PL => "Wybierz adapter sieciowy do inspekcji",
        Language::DE => "Wähle einen Netzwerkadapter zum inspizieren aus",
        Language::UK => "Вибрати мережевий адаптер для інспекції",
        Language::ZH => "选择需要监控的网络适配器",
        Language::RO => "Selectați adaptor de rețea pentru a inspecta",
        Language::KO => "검사할 네트워크 어댑터 선택",
        Language::PT_BR => "Selecione o adaptador de rede a ser inspecionado",
    })
}

pub fn application_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Application protocol",
        Language::IT => "Protocollo applicativo",
        Language::FR => "Protocole applicatif",
        Language::ES => "Protocolo de aplicación",
        Language::PL => "Protokół aplikacji",
        Language::DE => "Anwendungs-Protokoll",
        Language::UK => "Протокол аплікації",
        Language::ZH => "目标应用层协议",
        Language::RO => "Protocol aplicație",
        Language::KO => "어플리케이션 프로토콜",
        Language::PT_BR => "Protocolo de aplicação",
    }
}

pub fn select_filters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
        Language::PL => "Wybierz filtry, które mają być zastosowane na ruchu sieciowym",
        Language::DE => "Wähle die Filter, die auf den Netzwerkverkehr angewendet werden sollen",
        Language::UK => "Вибрати фільтри, які мають бути застосовані до мережевого трафіку",
        Language::ZH => "选择需要监控的目标",
        Language::RO => "Selectați filtre pentru traficul de rețea",
        Language::KO => "네트워크 트레픽에 적용할 필터 선택",
        Language::PT_BR => "Selecione os filtros a serem aplicados no tráfego de rede",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::RO | Language::KO => "Start!",
        Language::IT => "Avvia!",
        Language::FR => "Commencer!",
        Language::ES => "¡Empieza!",
        Language::PL => "Rozpocznij!",
        Language::UK => "Почати!",
        Language::ZH => "开始!",
        Language::PT_BR => "Começar!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddress:",
        Language::IT => "\nIndirizzo:",
        Language::FR | Language::DE => "\nAdresse:",
        Language::ES => "\nDirección:",
        Language::PL => "\nAdres:",
        Language::UK => "\nАдреса:",
        Language::ZH => "\n网络地址:",
        Language::RO => "\nAdresă:",
        Language::KO => "\n주소:",
        Language::PT_BR => "\nEndereço:",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddresses:",
        Language::IT => "\nIndirizzi:",
        Language::FR => "\nAdresses:",
        Language::ES => "\nDirecciones:",
        Language::PL => "\nAdresy:",
        Language::DE => "\nAdressen:",
        Language::UK => "\nАдреси:",
        Language::ZH => "\n网络地址:",
        Language::RO => "\nAdrese:",
        Language::KO => "\n주소:",
        Language::PT_BR => "\nEndereços:",
    }
}

pub fn ip_version_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
        Language::FR => "Version IP",
        Language::ES => "Versión IP",
        Language::PL => "Wersja IP",
        Language::DE => "IP Version",
        Language::UK => "Версія IP",
        Language::ZH => "目标IP协议版本",
        Language::RO => "Versiune IP",
        Language::KO => "IP 버전",
        Language::PT_BR => "Versão de IP",
    })
}

pub fn transport_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transport protocol",
        Language::IT => "Protocollo di trasporto",
        Language::FR => "Protocole de transport",
        Language::ES => "Protocolo de transporte",
        Language::PL => "Protokół transportowy",
        Language::DE => "Netzwerkprotokoll",
        Language::UK => "Транспортний протокол",
        Language::ZH => "目标传输协议",
        Language::RO => "Protocol de transport",
        Language::KO => "전송 프로토콜",
        Language::PT_BR => "Protocolo de transporte",
    }
}

pub fn traffic_rate_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Traffic rate:",
        Language::IT => "Intensità del traffico:",
        Language::FR => "Fréquence du traffic:",
        Language::ES => "Tasa de tráfico:",
        Language::PL => "Prędkość ruchu:",
        Language::DE => "Daten Frequenz:",
        Language::UK => "Швидкість руху:",
        Language::ZH => "网络速率图:",
        Language::RO => "Rata de trafic:",
        Language::KO => "트레픽 속도:",
        Language::PT_BR => "Taxa de tráfego:",
    })
}

pub fn relevant_connections_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Relevant connections:",
        Language::IT => "Connessioni rilevanti:",
        Language::FR => "Connexions pertinentes:",
        Language::ES => "Conexiones Relevantes:",
        Language::PL => "Istotne połączenia:",
        Language::DE => "Relevante Verbindungen:",
        Language::UK => "Важливі підключення:",
        Language::ZH => "连接详情:",
        Language::RO => "Conexiuni relevante:",
        Language::KO => "관련 연결:",
        Language::PT_BR => "Conexões relevantes:",
    })
}

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::IT => "Impostazioni",
        Language::FR => "Paramètres",
        Language::ES => "Ajustes",
        Language::PL => "Ustawienia",
        Language::DE => "Einstellungen",
        Language::UK => "Налаштування",
        Language::ZH => "设置",
        Language::RO => "Setări",
        Language::KO => "설정",
        Language::PT_BR => "Configurações",
    }
}

pub fn yes_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::IT => "Sì",
        Language::FR => "Oui",
        Language::ES => "Sí",
        Language::PL => "Tak",
        Language::DE => "Ja",
        Language::UK => "Так",
        Language::ZH => "是",
        Language::RO => "Da",
        Language::KO => "네",
        Language::PT_BR => "Sim",
    })
}

pub fn ask_quit_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
        Language::PL => "Czy na pewno chcesz zakończyć analizę?",
        Language::DE => "Bist du sicher, dass du diese Analyse beenden willst?",
        Language::UK => "Чи справді хочеш закінчити аналіз?",
        Language::ZH => "您确定退出当前监控吗?",
        Language::RO => "Sunteți sigur că doriți să renunțați la această analiză?",
        Language::KO => "정말로 분석을 종료하겠습니까?",
        Language::PT_BR => "Tem certeza que deseja sair dessa análise?",
    })
}

pub fn quit_analysis_translation(language: Language) -> String {
    match language {
        Language::EN => "Quit analysis".to_string(),
        Language::IT => "Interrompi analisi".to_string(),
        Language::FR => "Quitter l'analyse".to_string(),
        Language::ES => "Quitar el análisis".to_string(),
        Language::PL => "Zakończ analize".to_string(),
        Language::DE => "Analyse beenden".to_string(),
        Language::UK => "Закінчити аналіз".to_string(),
        Language::ZH => "退出监控".to_string(),
        Language::RO => "Renunță la analiză".to_string(),
        Language::KO => "분석종료".to_string(),
        Language::PT_BR => "Sair da análise".to_string(),
    }
}

pub fn ask_clear_all_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
        Language::PL => "Czy na pewno chcesz wyczyścić powiadomienia?",
        Language::DE => "Bist du sicher, dass du alle Benachrichtigungen löschen willst?",
        Language::UK => "Чи справді хочеш видалити всі повідомлення?",
        Language::ZH => "确定清除所有通知?",
        Language::RO => "Sigur doriți să ștergeți notificările?",
        Language::KO => "알림을 삭제하시겠습니까?",
        Language::PT_BR => "Tem certeza que deseja limpar todas as notificações?",
    })
}

pub fn clear_all_translation(language: Language) -> String {
    match language {
        Language::EN => "Clear all".to_string(),
        Language::IT => "Elimina tutte".to_string(),
        Language::FR => "Tout effacer".to_string(),
        Language::ES => "Borrar todo".to_string(),
        Language::PL => "Wyczyść wszystko".to_string(),
        Language::DE => "Alle leeren".to_string(),
        Language::UK => "Видалити все".to_string(),
        Language::ZH => "清除所有".to_string(),
        Language::RO => "Ștergeți tot".to_string(),
        Language::KO => "모두 지우기".to_string(),
        Language::PT_BR => "Limpar tudo".to_string(),
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::IT => "Nascondi",
        Language::FR => "Masquer",
        Language::ES => "Ocultar",
        Language::PL => "Ukryj",
        Language::DE => "Verstecken",
        Language::UK => "Заховати",
        Language::ZH => "隐藏",
        Language::RO => "Ascundeți",
        Language::KO => "숨기기",
        Language::PT_BR => "Esconder",
    }
}

pub fn no_addresses_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."),
        Language::IT => format!("Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."),
        Language::FR => format!("Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."),
        Language::ES => format!("No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."),
        Language::PL => format!("Nie można zaobserwować żadnego ruchu, ponieważ wybrany adapter nie ma aktywnych adresów...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Jeśli jesteś pewien, że jesteś podłączony do internetu, spróbuj wybrać inny adapter."),
        Language::DE => format!("Es kann kein Netzwerkverkehr beobachtet werden, weil der Adapter keine aktiven Adressen hat...\n\n\
                                 Netzwerkadapter: {adapter}\n\n\
                                 Wenn du dir sicher bist, dass du mit dem Internet verbunden bist, probier einen anderen Adapter auszuwählen."),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку тому що вибраний адаптер немає активних адрес... \n\n\
                                 Мережквий адаптер: {adapter}\n\n\
                                 Якщо Ти впевнений, що підключений до інтернету, спробуй вибрати інший адаптер."),
        Language::ZH => format!("您选择的网络适配器当前无活动网络...\n\n\
                                网络适配器: {adapter}\n\n\
                                如果您确信您已成功连接互联网, 请尝试选择其他网络适配器."),
        Language::RO => format!("Niciun trafic nu poate fi observat deoarece adaptorul selectat nu are adrese active...\n\n\
                                Adaptor de rețea: {adapter}\n\n\
                                Dacă sunteți sigur că sunteți conectat la internet, încercați să alegeți un alt adaptor."),
        Language::KO => format!("선택한 어댑터에 유효한 주소가 없기 때문에 트래픽을 확인할 수 없습니다...\n\n\
                                네트워크 어뎁터: {adapter}\n\n\
                                인터넷이 연결되어있다면 다른 어댑터로 시도해보세요."),
        Language::PT_BR => format!("Nenhum tráfego pôde ser observado pois o adaptador selecionado não tem endereços ativos...\n\n\
                                Adaptador de rede: {adapter}\n\n\
                                Se você tem certeza que está conectado à internet, tente escolher outro adaptador."),
    })
}

pub fn waiting_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"),
        Language::IT => format!("Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"),
        Language::FR => format!("Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"),
        Language::ES => format!("Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"),
        Language::PL => format!("Nie zaobserowano żadnego ruchu sieciowego. Oczekiwanie na pakiety...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Czy na pewno jesteś podłączony do internetu i wybrałeś właściwy adapter?"),
        Language::DE => format!("Noch kein Netzwerkverkehr beobachtet. Warte auf Pakete...\n\n\
                                 Netzwerkadapter: {adapter}\n\n\
                                 Bist du sicher, dass du mit dem Internet verbunden bist und den richtigen Adapter ausgewählt hast?"),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку. Очікування на пакети...\n\n\
                                 Мережквий адаптер: {adapter}\n\n\
                                 Чи Ти дійсно підключений до інтернету і вибрав відповідний мережевий адаптер?"),
        Language::ZH => format!("暂无流量数据. 等待网络活动中......\n\n\
                                 网络适配器: {adapter}\n\n\
                                 您确信您已成功连接到互联网, 并选择了当前正在使用的的网络适配器吗?"),
        Language::RO => format!("Nu a fost observat încă trafic. Se așteaptă pachetele de rețea...\n\n\
                                Adaptor de rețea: {adapter}\n\n\
                                Ești sigur că ești conectat la internet și ai selectat adaptorul corect?"),
        Language::KO => format!("아직 트래픽이 관찰되지 않았습니다. 네트워크 패킷 대기 중...\n\n\
                                네트워크 어뎁터: {adapter}\n\n\
                                인터넷에 연결되어 있고 올바른 어댑터를 선택하셨습니까?"),
        Language::PT_BR => format!("Nenhum tráfego foi observado ainda. Esperando por pacotes de rede...\n\n\
                                Adaptador de rede: {adapter}\n\n\
                                Você tem certeza que está conectado à internet e selecionou o adaptador correto?"),
    })
}

pub fn some_observed_translation(
    language: Language,
    observed: &str,
    filters: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{filters}"),
        Language::IT => format!("Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati...\n\n{filters}"),
        Language::FR => format!("Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés...\n\n{filters}"),
        Language::ES => format!("Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados...\n\n{filters}"),
        Language::PL => format!("Suma przechwyconych pakietów: {observed}\n\n\
                                 Przefiltrowane pakiety: 0\n\n\
                                 Niektóre pakiety zostały przechwycone, ale żaden nie został wybrany zgodnie z wskazanymi filtrami...\n\n{filters}"),
        Language::DE => format!("Anzahl der empfangenen Pakete: {observed}\n\n\
                                 gefilterte Pakete: 0\n\n\
                                 Ein Paar Pakete wurden empfangen, aber es entsprechen noch keine den spezifizierten Filtern...\n\n{filters}"),
        Language::UK => format!("Сума перехоплених пакетів: {observed}\n\n\
                                 Відфільтровані пакеті: 0\n\n\
                                 Деякі пакети були перехоплені, але жоден з них не був вибраний відповідно до вказаних фільтрів...\n\n{filters}"),
        Language::ZH => format!("监测到的数据包总数: {observed}\n\n\
                                 目标数据包总数: 0\n\n\
                                 当前已监测到一些数据包, 但其中并未包含您的目标数据包......\n\n{filters}"),
        Language::RO => format!("Total pachete interceptate: {observed}\n\n\
                                Pachete filtrate: 0\n\n\
                                Unele pachete au fost interceptate, dar încă niciunul nu a fost selectat conform filtrelor pe care le-ați specificat...\n\n{filters}"),
        Language::KO => format!("감지한 총 패킷: {observed}\n\n\
                                필터링된 패킷: 0\n\n\
                                일부 패킷이 감지되었지만, 지정한 필터에 따라 선택되지 않았습니다...\n\n{filters}"),
        Language::PT_BR => format!("Total de pacotes interceptados: {observed}\n\n\
                                 Pacotes filtrados: 0\n\n\
                                 Alguns pacotes foram interceptados, mas nenhum de acordo com os filtros especificados...\n\n{filters}"),
    })
}

pub fn filtered_packets_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered packets:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Pacchetti filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Paquets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Paquetes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::PL => format!("Przefiltrowane pakiety:\n   {filtered} ({percentage} z całości)"),
        Language::DE => format!("Gefilterte Pakete:\n   {filtered} ({percentage} der Gesamtzahl)"),
        Language::UK => {
            format!("Відфільтровані пакети:\n   {filtered} ({percentage} від загальної суми)")
        }
        Language::ZH => {
            format!("目标数据包计数:\n   {filtered} (占所有数据包的 {percentage})")
        }
        Language::RO => {
            format!("Pachete filtrate:\n   {filtered} ({percentage} din total)")
        }
        Language::KO => {
            format!("필터링된 패킷:\n   {filtered} ({percentage} 의 일부)")
        }
        Language::PT_BR => format!("Pacotes filtrados:\n   {filtered} ({percentage} do total)"),
    })
}

pub fn filtered_bytes_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered bytes:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Byte filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Octets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Bytes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::PL => format!("Przechwycone bajty:\n   {filtered} ({percentage} całości)"),
        Language::DE => format!("Gefilterte Bytes:\n   {filtered} ({percentage} der Gesamtzahl)"),
        Language::UK => {
            format!("Відфільтровані байти:\n   {filtered} ({percentage} від загальної суми)")
        }
        Language::ZH => {
            format!("目标网络流量计数:\n   {filtered} (占所有网络流量的 {percentage})")
        }
        Language::RO => {
            format!("Octeți filtrați:\n   {filtered} ({percentage} din total)")
        }
        Language::KO => {
            format!("필터링된 바이트:\n   {filtered} ({percentage} 의 일부)")
        }
        Language::PT_BR => format!("Bytes filtrados:\n   {filtered} ({percentage} do total)"),
    })
}

pub fn filtered_application_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Filtered packets per application protocol:",
        Language::IT => "Pacchetti filtrati per protocollo applicativo:",
        Language::FR => "Paquets filtrés par protocole applicatif:",
        Language::ES => "Paquetes filtrados por protocolo de aplicación:",
        Language::PL => "Przefiltrowane pakiety według protokołu aplikacji:",
        Language::DE => "Gefilterte Pakete je Anwendungs-Protokoll:",
        Language::UK => "Відфільтровані пакети протоколу аплікації/програми:",
        Language::ZH => "按应用层协议分类的目标数据包计数:",
        Language::RO => "Pachete filtrate pe protocol de aplicație:",
        Language::KO => "애플리케이션 프로토콜당 필터링된 패킷 수:",
        Language::PT_BR => "Pacotes filtrados por protocolo de aplicação:",
    })
}

pub fn no_favorites_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Nothing to show at the moment.\n\
                         To add a connection to your favorites, click on the star symbol near the connection.",
        Language::IT => "Nulla da vedere per il momento.\n\
                         Per aggiungere una connessione ai tuoi preferiti, clicca sul simbolo della stella vicino alla connessione.",
        Language::FR => "Rien a voir pour le moment.\n\
                         Pour ajouter une connexion à vos favoris, cliquez sur l'étoile à côté de la connexion.",
        Language::ES => "Nada que mostrar por el momento.\n\
                         Para añadir una conexión a sus favoritos, haga clic en el símbolo de la estrella situado junto a la conexión.",
        Language::PL => "Nie ma nic do pokazania w tej chwili.\n\
                         Aby dodać połączenie do ulubionych, kliknij na ikonę 'gwiazdki' obok połączenia.",
        Language::DE => "Im Moment nichts zu zeigen.\n\
                         Um eine Verbindung zu deinen Favoriten hinzuzufügen, klick das auf das Stern-Symbol neben der Verbindung.",
        Language::UK => "Немає, що показати в цей момент.\n\
                         Щоб додати підключення до улюблених, натисни на іконку 'зірочки' біля підключення.",
        Language::ZH => "收藏夹还是空的.\n\
                         小贴士: 点击连接右侧的小星星即可收藏到这里哦.",
        Language::RO => "Nimic de arătat în acest moment.\n\
                        Pentru a adăuga o conexiune la favorite, faceți clic pe simbolul stea din apropierea conexiunii.",
        Language::KO => "현재는 보여줄게 없습니다.\n\
                         즐겨찾기에 연결을 추가하려면 별을 눌러주세요.",
        Language::PT_BR => "Nada a ser exibido ainda.\n\
                        Para adicionar uma conexão aos seus favoritos, clique no símbolo de estrela próximo da conexão.",
    })
}

pub fn error_translation(language: Language, error: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
        Language::PL => format!(
            "Wystąpił błąd! \n\n\
                                {error}"
        ),
        Language::DE => format!(
            "Es ist ein Fehler aufgetreten! \n\n\
                                {error}"
        ),
        Language::UK => format!(
            "Виступила помилка! \n\n\
                                {error}"
        ),
        Language::ZH => format!(
            "发生了一些错误! \n\n\
                                {error}"
        ),
        Language::RO => format!(
            "A apărut o eroare! \n\n\
                                {error}"
        ),
        Language::KO => format!(
            "오류가 발생하였습니다! \n\n\
                                {error}"
        ),
        Language::PT_BR => format!(
            "Um erro ocorreu! \n\n\
                                {error}"
        ),
    })
}

pub fn both_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "both",
        Language::IT => "entrambi",
        Language::FR => "les deux",
        Language::ES | Language::PT_BR => "ambos",
        Language::PL => "oba",
        Language::DE => "beide",
        Language::UK => "обидва",
        Language::ZH => "皆需",
        Language::RO => "ambele",
        Language::KO => "둘다",
    }
}

// pub fn all_protocols_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All protocols",
//         Language::IT => "Tutti i protocolli",
//         Language::FR => "Tous les protocoles",
//         Language::ES => "Todos los protocolos",
//         Language::PL => "Wszystkie protokoły",
//         Language::DE => "Alle Protokolle",
//         Language::PT => "Todos os pacotes",
//     }
// }

pub fn all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "All",
        Language::IT => "Tutti",
        Language::FR => "Tous",
        Language::ES | Language::PT_BR => "Todos",
        Language::PL => "Wszystkie",
        Language::DE => "Alle",
        Language::UK => "Усі",
        Language::ZH => "所有",
        Language::RO => "Toate",
        Language::KO => "모두",
    }
}

pub fn packets_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets per second",
        Language::IT => "pacchetti al secondo",
        Language::FR => "paquets par seconde",
        Language::ES => "paquetes por segundo",
        Language::PL => "pakiety na sekundę",
        Language::DE => "pakete pro Sekunde",
        Language::UK => "пакети на секунду",
        Language::ZH => "数据包",
        Language::RO => "pachete pe secundă",
        Language::KO => "초당 패킷",
        Language::PT_BR => "pacotes por segundo",
    }
}

pub fn bytes_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "bytes per second",
        Language::IT => "byte al secondo",
        Language::FR => "octets par seconde",
        Language::ES | Language::PT_BR => "bytes por segundo",
        Language::PL => "bajty na sekundę",
        Language::DE => "bytes pro Sekunde",
        Language::UK => "байти на секунду",
        Language::ZH => "网络流量",
        Language::RO => "octeți pe secundă",
        Language::KO => "초당 바이트",
    }
}

pub fn recent_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most recent",
        Language::IT => "più recenti",
        Language::FR => "la plus récente",
        Language::ES => "más reciente",
        Language::PL => "najnowsze",
        Language::DE => "zuletzt",
        Language::UK => "найновіші",
        Language::ZH => "按时间",
        Language::RO => "cea mai recentă",
        Language::KO => "가장 최근",
        Language::PT_BR => "mais recentes",
    }
}

pub fn packets_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most packets",
        Language::IT => "più pacchetti",
        Language::FR => "le plus de paquets",
        Language::ES => "mayoría de los paquetes",
        Language::PL => "najwięcej pakietów",
        Language::DE => "meiste Pakete",
        Language::UK => "найбільше пакетів",
        Language::ZH => "按数据包",
        Language::RO => "cele mai multe pachete",
        Language::KO => "대부분의 패킷",
        Language::PT_BR => "maioria dos pacotes",
    }
}

pub fn bytes_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most bytes",
        Language::IT => "più byte",
        Language::FR => "le plus de données",
        Language::ES => "mayoría de los bytes",
        Language::PL => "najwięcej bajtów",
        Language::DE => "meiste Bytes",
        Language::UK => "найбільше байтів",
        Language::ZH => "按流量",
        Language::RO => "cei mai mulți octeți",
        Language::KO => "대부분의 바이트",
        Language::PT_BR => "maioria dos bytes",
    }
}

pub fn favorite_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "favorites",
        Language::IT => "preferiti",
        Language::FR => "favoris",
        Language::ES | Language::PT_BR => "favoritos",
        Language::PL => "ulubione",
        Language::DE => "Favoriten",
        Language::UK => "улюблені",
        Language::ZH => "收藏夹",
        Language::RO => "favorite",
        Language::KO => "즐겨찾기",
    }
}

pub fn notifications_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::FR => "Personnalisez vos notifications",
        Language::ES => "Personaliza tus notificaciones",
        Language::PL => "Dostosuj powiadomienia",
        Language::DE => "Personalisier deine Benachrichtigungen",
        Language::UK => "Достосуй повідомлення",
        Language::ZH => "自定义通知",
        Language::RO => "Personalizați-vă notificările",
        Language::KO => "사용자 지정 알림",
        Language::PT_BR => "Personalize suas notificações",
    })
}

pub fn appearance_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::IT => "Scegli il tuo tema preferito",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::ES => "Elige tu tema favorito",
        Language::PL => "Wybierz swój ulubiony motyw",
        Language::DE => "Wähl dein Lieblingsdesign",
        Language::UK => "Вибери улюблену тему",
        Language::ZH => "选择您喜欢的主题",
        Language::RO => "Selectați tema preferată",
        Language::KO => "태마를 선택하세요",
        Language::PT_BR => "Escolha seu tema favorito",
    })
}

pub fn languages_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select your language",
        Language::IT => "Seleziona la lingua",
        Language::FR => "Sélectionnez votre langue",
        Language::ES => "Selecciona tu idioma",
        Language::PL => "Wybierz język",
        Language::DE => "Stell deine Sprache ein",
        Language::UK => "Вибери мову",
        Language::ZH => "选择显示语言",
        Language::RO => "Selectați limba",
        Language::KO => "언어를 선택하세요",
        Language::PT_BR => "Selecione seu idioma",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters:",
        Language::IT => "Filtri attivi:",
        Language::FR => "Filtres actifs",
        Language::ES => "Filtros activos:",
        Language::PL => "Aktywne filtry:",
        Language::DE => "Aktive Filter:",
        Language::UK => "Активні фільтри:",
        Language::ZH => "活动的过滤器:",
        Language::RO => "Filtre active:",
        Language::KO => "활성화된 필터:",
        Language::PT_BR => "Filtros ativos:",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::IT => "nessuno",
        Language::FR => "aucun",
        Language::ES => "ninguno",
        Language::PL => "brak",
        Language::DE => "keine",
        Language::UK => "бракує",
        Language::ZH => "无",
        Language::RO => "niciunul",
        Language::KO => "없음",
        Language::PT_BR => "nenhum",
    }
}

pub fn yeti_night_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original dark theme",
        Language::IT => "Il tema scuro originale di Sniffnet",
        Language::FR => "Thème original sombre de Sniffnet",
        Language::ES => "Tema oscuro original de Sniffnet",
        Language::PL => "Oryginalny, ciemny motyw Sniffnet",
        Language::DE => "Sniffnets urspüngliches, dunkles Design",
        Language::UK => "Оригінальний, темний мотив Sniffnet",
        Language::ZH => "Sniffnet暗黑",
        Language::RO => "Tema întunecată originală Sniffnet",
        Language::KO => "Sniffnet의 기본 다크테마",
        Language::PT_BR => "Tema escuro original de Sniffnet",
    }
}

pub fn yeti_day_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original light theme",
        Language::IT => "Il tema chiaro originale di Sniffnet",
        Language::FR => "Thème original clair de Sniffnet",
        Language::ES | Language::PT_BR => "Tema claro original de Sniffnet",
        Language::PL => "Oryginalny, jasny motyw Sniffnet",
        Language::DE => "Sniffnets urspüngliches, helles Design",
        Language::UK => "Оригінальний, світлий мотив Sniffnet",
        Language::ZH => "Sniffnet浅色",
        Language::RO => "Tema deschisă originală Sniffnet",
        Language::KO => "Sniffnet의 기본 라이트테마",
    }
}

pub fn deep_sea_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "To dive into network traffic",
        Language::IT => "Per immergersi nel traffico di rete",
        Language::FR => "Pour plonger dans votre trafic réseau",
        Language::ES => "Para sumergirse en el tráfico de la red",
        Language::PL => "Aby zanurzyć się w ruchu sieciowym",
        Language::DE => "Um in den Netzwerkverkehr einzutauchen",
        Language::UK => "Проаналізувати мережевий рух",
        Language::ZH => "潜入网络活动的海洋",
        Language::RO => "Pentru a vă scufunda în traficul de rețea",
        Language::KO => "네트워크 트레픽으로 바로가기",
        Language::PT_BR => "Para mergulhar-se no tráfego de rede",
    }
}

pub fn mon_amour_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Lovely theme made for dreamers",
        Language::IT => "Tema incantevole fatto per i sognatori",
        Language::FR => "Thème romantique fait pour les rêveurs",
        Language::ES => "Tema encantador hecho para soñadores",
        Language::PL => "Uroczy motyw stworzony dla marzycieli",
        Language::DE => "Liebevolles Design für Träumer",
        Language::UK => "Прекрасна тема для мрійників",
        Language::ZH => "梦想家的主题",
        Language::RO => "O temă minunată creată pentru visători",
        Language::KO => "사랑스러운 몽환가들을 위한 테마",
        Language::PT_BR => "Um encantador tema feito por sonhadores",
    }
}

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::IT => "In entrata",
        Language::FR => "Entrant",
        Language::ES => "Entrante",
        Language::PL => "Przychodzące",
        Language::DE => "Ankommend",
        Language::UK => "Вхідні",
        Language::ZH => "入站",
        Language::RO => "de intrare",
        Language::KO => "수신중",
        Language::PT_BR => "de entrada",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::IT => "In uscita",
        Language::FR => "Sortant",
        Language::ES => "Saliente",
        Language::PL => "Wychodzące",
        Language::DE => "Ausgehend",
        Language::UK => "Вихідні",
        Language::ZH => "出站",
        Language::RO => "de ieșire",
        Language::KO => "발신중",
        Language::PT_BR => "de saída",
    }
}

pub fn notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Notifications",
        Language::IT => "Notifiche",
        Language::ES => "Notificaciones",
        Language::PL => "Powiadomienia",
        Language::DE => "Benachrichtigungen",
        Language::UK => "Повідомлення",
        Language::ZH => "通知",
        Language::RO => "Notificări",
        Language::KO => "알림",
        Language::PT_BR => "Notificações",
    }
}

pub fn style_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Style",
        Language::IT => "Stile",
        Language::ES | Language::PT_BR => "Estilo",
        Language::PL => "Styl",
        Language::DE | Language::RO => "Stil",
        Language::UK => "Стиль",
        Language::ZH => "主题",
        Language::KO => "스타일",
    }
}

pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::IT => "Lingua",
        Language::FR => "Langue",
        Language::ES | Language::PT_BR => "Idioma",
        Language::PL => "Język",
        Language::DE => "Sprache",
        Language::UK => "Мова",
        Language::ZH => "语言",
        Language::RO => "Limbă",
        Language::KO => "언어",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::IT => "Panoramica",
        Language::FR => "Résumé",
        Language::ES => "Resumen",
        Language::PL => "Przegląd",
        Language::DE => "Übersicht",
        Language::UK => "Огляд",
        Language::ZH => "概览",
        Language::RO => "Prezentare generală",
        Language::KO => "개요",
        Language::PT_BR => "Visão geral",
    }
}

// pub fn inspect_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Inspect",
//         Language::IT => "Ispeziona",
//         Language::FR => "Inspecter",
//         Language::ES => "Inspeccionar",
//         Language::PL => "Sprawdź",
//         Language::DE => "Überprüfen",
//         Language::PT => "Inspecionar",
//     }
// }

pub fn packets_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a packets threshold is exceeded",
        Language::IT => "Notificami quando una soglia di pacchetti è superata",
        Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
        Language::ES => "Notificarme cuando se supere un límite de paquetes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg pakietów",
        Language::DE => "Benachrichtige mich, wenn die Pakete eine Schwelle überschreiten",
        Language::UK => "Повідом мене про переліміт пакетів",
        Language::ZH => "超过设定的数据包数量阈值时通知我",
        Language::RO => "Anunță-mă când este depășit un prag de pachete",
        Language::KO => "패킷 임계값을 초과하면 알림",
        Language::PT_BR => "Notificar-me quando for excedido o limite de pacotes",
    }
}

pub fn bytes_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a bytes threshold is exceeded",
        Language::IT => "Notificami quando una soglia di byte è superata",
        Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
        Language::ES => "Notificarme cuando se exceda un límite de bytes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg bajtów",
        Language::DE => "Benachrichtige mich, wenn die Bytes eine Schwelle überschreiten",
        Language::UK => "Повідом мене про переліміт байтів",
        Language::ZH => "超过设定的网络流量阈值时通知我",
        Language::RO => "Anunță-mă când este depășit un prag de octeți",
        Language::KO => "바이트 임계값을 초과하면 알림",
        Language::PT_BR => "Notificar-me quando for excedido o limite de bytes",
    }
}

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::IT => "(al secondo)",
        Language::FR => "(par seconde)",
        Language::ES | Language::PT_BR => "(por segundo)",
        Language::PL => "(na sekundę)",
        Language::DE => "(pro Sekunde)",
        Language::UK => "(на секунду)",
        Language::ZH => "(每秒) ",
        Language::RO => "(pe secundă)",
        Language::KO => "(초당)",
    }
}

pub fn specify_multiples_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "; you can also specify 'K', 'M' and 'G'",
        Language::IT => "; puoi anche specificare 'K', 'M' e 'G'",
        Language::FR => "; vous pouvez également spécifier 'K', 'M' et 'G'",
        Language::ES => "; también puede especificar 'K', 'M' y 'G'",
        Language::PL => "; możesz również określić 'K', 'M' i 'G'",
        Language::DE => "; du kannst auch 'K', 'M' und 'G' festlegen",
        Language::UK => "; можеш також вибрати 'K', 'M' i 'G'",
        Language::ZH => "您可指定 'K', 'M', 'G'",
        Language::RO => "; puteți specifica 'K', 'M', 'G'",
        Language::KO => "; 지정가능합니다 'K', 'M', 'G'",
        Language::PT_BR => "; também é possível especificar 'K', 'M' e 'G'",
    }
}

pub fn favorite_notification_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when new data are exchanged from my favorites",
        Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
        Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
        Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
        Language::PL => "Powiadom mnie, gdy nowe dane z moich ulubionych zostaną wymienione",
        Language::DE => {
            "Benachrichtige mich, wenn neue Daten mit meinen Favoriten ausgetauscht werden"
        }
        Language::UK => "Повідом мене, коли буде обмін даними з моїх улюблених",
        Language::ZH => "收藏夹内的连接有新活动时通知我",
        Language::RO => "Anunță-mă când sunt transferate date noi de la favoritele mele",
        Language::KO => "즐겨찾기에서 새 데이터가 교환될 때 알림",
        Language::PT_BR => "Notificar-me quando transferir novos dados de meus favoritos",
    }
}

pub fn threshold_translation(language: Language) -> String {
    match language {
        Language::EN => "Threshold: ".to_string(),
        Language::IT => "Soglia: ".to_string(),
        Language::FR => "Seuil: ".to_string(),
        Language::ES => "Límite: ".to_string(),
        Language::PL => "Próg: ".to_string(),
        Language::DE => "Schwellenwert: ".to_string(),
        Language::UK => "Ліміт: ".to_string(),
        Language::ZH => "阈值: ".to_string(),
        Language::RO => "Prag: ".to_string(),
        Language::KO => "임계값: ".to_string(),
        Language::PT_BR => "Limite: ".to_string(),
    }
}

pub fn volume_translation(language: Language, value: u8) -> String {
    match language {
        Language::EN | Language::IT | Language::FR => format!("Volume: {value:^3}%"),
        Language::ES => format!("Volumen: {value:^3}%"),
        Language::PL => format!("Głośność: {value:^3}%"),
        Language::DE => format!("Lautstärke: {value:^3}%"),
        Language::UK => format!("Гучність: {value:^3}%"),
        Language::ZH => format!("通知音量: {value:^3}%"),
        Language::RO => format!("Volum: {value:^3}%"),
        Language::KO => format!("볼륨: {value:^3}%"),
        Language::PT_BR => format!("Volume: {value:^3}%"),
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound:",
        Language::IT => "Suono:",
        Language::FR => "Son:",
        Language::ES => "Sonido:",
        Language::PL => "Dźwięk:",
        Language::DE => "Ton:",
        Language::UK => "Звук:",
        Language::ZH => "通知音:",
        Language::RO => "Sunet:",
        Language::KO => "사운드:",
        Language::PT_BR => "Som:",
    }
}

pub fn open_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Open full report",
        Language::IT => "Apri report completo",
        Language::FR => "Ouvrir le rapport complet",
        Language::ES => "Abrir el informe completo",
        Language::PL => "Otwórz pełny raport",
        Language::DE => "Kompletten Bericht öffnen",
        Language::UK => "Відкрий повний рапорт",
        Language::ZH => "打开完整报告",
        Language::RO => "Deschideți raport complet",
        Language::KO => "전체 보고서 열기",
        Language::PT_BR => "Abrir relatório completo",
    }
}

pub fn bytes_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bytes threshold exceeded!",
        Language::IT => "Soglia di Byte superata!",
        Language::FR => "Seuil de donnée atteint!",
        Language::ES => "¡Límite de bytes superado!",
        Language::PL => "Próg bajtów przekroczony!",
        Language::DE => "Byte-Schwellenwert überschritten!",
        Language::UK => "Ліміт байтів перевищено!",
        Language::ZH => "达到设定的网络流量阈值!",
        Language::RO => "Prag de octeți depășit!",
        Language::KO => "바이트 임계값 초과!",
        Language::PT_BR => "O limite de bytes foi excedido!",
    }
}

pub fn bytes_exceeded_value_translation(language: Language, value: &str) -> String {
    let trimmed_value = value.trim();
    match language {
        Language::EN => format!("{trimmed_value} bytes have been exchanged"),
        Language::IT => format!("{trimmed_value} byte sono stati scambiati"),
        Language::FR => format!("{trimmed_value} octets ont été échangé"),
        Language::ES => format!("{trimmed_value} byte/s han sido intercambiado/s"),
        Language::PL => format!("Wymieniono {trimmed_value} bajtów"),
        Language::DE => format!("{trimmed_value} Bytes wurden ausgetauscht"),
        Language::UK => format!("{trimmed_value} байтів було обміняно"),
        Language::ZH => format!("已交换字节 {trimmed_value}"),
        Language::RO => format!("au fost transferați {trimmed_value} octeți"),
        Language::KO => format!("바이트 {trimmed_value} 가 교환되었습니다"),
        Language::PT_BR => format!("{trimmed_value} bytes foram transferidos"),
    }
}

pub fn packets_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Packets threshold exceeded!",
        Language::IT => "Soglia di pacchetti superata!",
        Language::FR => "Le seuil de paquet a été atteint!",
        Language::ES => "¡Se ha superado el límite de paquetes!",
        Language::PL => "Próg pakietów przekroczony!",
        Language::DE => "Paket-Schwellenwert überschritten!",
        Language::UK => "Ліміт пакетів перевищено!",
        Language::ZH => "达到设定的数据包数量阈值!",
        Language::RO => "Prag de pachete depășit!",
        Language::KO => "패킷 임계값 초과!",
        Language::PT_BR => "O limite de pacotes foi excedido!",
    }
}

pub fn packets_exceeded_value_translation(language: Language, value: u32) -> String {
    match language {
        Language::EN => format!("{value} packets have been exchanged"),
        Language::IT => format!("{value} pacchetti sono stati scambiati"),
        Language::FR => match value {
            1 => "1 paquet a été échangé".to_owned(),
            npackets => format!("{npackets} paquets ont été échangés"),
        },
        Language::ES => format!("{value} paquete/s han sido intercambiado/s"),
        Language::PL => format!("Wymieniono {value} pakietów"),
        Language::DE => format!("{value} Pakete wurden ausgetauscht"),
        Language::UK => format!("Обміняно {value} пакетів"),
        Language::ZH => format!("已交换数据包 {value}"),
        Language::RO => format!("au fost transferate {value} pachete"),
        Language::KO => format!("패킷 {value} 가 교환되었습니다"),
        Language::PT_BR => format!("{value} pacotes foram transferidos"),
    }
}

pub fn favorite_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from favorites!",
        Language::IT => "Nuovi dati scambiati dai preferiti!",
        Language::FR => "Nouvel échange de donnée depuis un favori!",
        Language::ES => "¡Nuevos datos intercambiados de favoritos!",
        Language::PL => "Nowe dane wymienione z ulubionych!",
        Language::DE => "Neue Daten mit den Favoriten ausgetauscht!",
        Language::UK => "Нові дані обміняно з улюблених!",
        Language::ZH => "收藏夹内的连接有新活动!",
        Language::RO => "Date noi transferate de la favorite!",
        Language::KO => "즐겨찾기에서 새 데이터 교환",
        Language::PT_BR => "Novos dados transferidos dos favoritos!",
    }
}

pub fn no_notifications_set_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "You haven't enabled notifications yet!\n\n\
                                 After you will enable them, this page will display a log of your notifications\n\n\
                                 You can enable notifications from settings:",
        Language::IT => "Non hai ancora abilitato le notifiche!\n\n\
                                Dopo che le avrai abilitate, questa pagina mostrerà una collezione delle tue notifiche\n\n\
                                Puoi abilitare le notifiche dalle impostazioni:",
        Language::FR => "Vous n'avez pas activé les notifications!\n\n\
                                    Une fois activées, cette page affichera le journal des notifications\n\n\
                                    Vous pouvez les activer dans les paramètres:",
        Language::ES => "¡Aún no has activado las notificaciones!\n\n\
                                 Después de activarlas, esta página mostrará un registro de sus notificaciones\n\n\
                                 Puedes activar las notificaciones desde los ajustes:",
        Language::PL => "Nie włączyłeś jeszcze powiadomień!\n\n\
                                 Po ich włączeniu, ta strona wyświetli dziennik twoich powiadomień\n\n\
                                 Możesz włączyć powiadomienia w ustawieniach:",
        Language::DE => "Benachrichtigungen wurden noch nicht aktiviert!\n\n\
                         Nachdem du sie aktiviert hast, wird diese Seite eine Liste deiner Benachrichtigungen anzeigen\n\n\
                         Du kannst die Benachrichtigungen in den Einstellungen aktivieren:",
        Language::UK => "Повідомлення не активовані!\n\n\
                                 Після їх активації, на цій сторінці побачиш список своїх повідомлень\n\n\
                                 Можеш вимкнути повідомлення в налаштуваннях:",
        Language::ZH => "您还没有设定任何通知!\n\n\
                                 启用它们后，此页面将显示您的通知日志\n\n\
                                 您可以从设置中设定:",
        Language::RO => "Încă nu ați activat notificările!\n\n\
                                 După ce le veți activa, această pagină va afișa un jurnal al notificărilor dvs\n\n\
                                 Puteți activa notificările din setări:",
        Language::KO => "아직 알림을 활성화하지 않았습니다!\n\n\
                                 활성화로 설정하면 이 페이지에 알림 로그가 표시됩니다\n\n\
                                 설정에서 알림을 활성화할 수 있습니다:",
        Language::PT_BR => "Você ainda não ativou as notificações!\n\n\
                                 Uma vez ativadas, essa página exibirá um registro de suas notificações\n\n\
                                 Você pode ativar as notificações pelas configurações:",
    })
}

pub fn no_notifications_received_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => {
            "Nothing to see at the moment...\n\n\
                                 When you will receive a notification, it will be displayed here"
        }
        Language::IT => {
            "Nulla da vedere al momento...\n\n\
                                Quando riceverai una notifica, essa verrà mostrata qui"
        }
        Language::FR => {
            "Rien à voir pour le moment...\n\n\
                                 Lorsque vous recevrez une notification, elle s'affichera ici"
        }
        Language::ES => {
            "Nada que ver por el momento...\n\n\
                                 Cuando reciba una notificación, aparecerá aquí"
        }
        Language::PL => {
            "Nic do wyświetlenia w tej chwili...\n\n\
                                 Gdy otrzymasz powiadomienie, pojawi się ono tutaj"
        }
        Language::DE => {
            "Im Moment nichts zu sehen...\n\n\
                                 Wenn du eine Benachrichtigung erhälst, wird sie hier angezeigt"
        }
        Language::UK => {
            "Немає що показати в даний момент...\n\n\
                                 Коли отримаєш повідомлення, побачиш його тут"
        }
        Language::ZH => {
            "还没有任何通知...\n\n\
                                 当您收到通知时，它会显示在这里"
        }
        Language::RO => {
            "Nimic de văzut momentan...\n\n\
                                 Când veți primi o notificare, aceasta va fi afișată aici"
        }
        Language::KO => {
            "현재는 볼 것이 없습니다...\n\n\
                                 알림을 받으면 여기에 표시됩니다"
        }
        Language::PT_BR => {
            "Nada para ser exibido ainda...\n\n\
                                 Quando receber uma notificação, ela será exibida aqui"
        }
    })
}

pub fn only_last_30_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the last 30 notifications are displayed",
        Language::IT => "Solo le ultime 30 notifiche sono mostrate",
        Language::FR => "Seulement les 30 dernières notifications sont affichées",
        Language::ES => "Sólo se muestran las últimas 30 notificaciones",
        Language::PL => "Wyświetlane jest tylko 30 ostatnich powiadomień",
        Language::DE => "Nur die letzten 30 Benachrichtigungen werden angezeigt",
        Language::UK => "Можеш побачити лише 30 останніх повідомлень",
        Language::ZH => "仅显示最近 30 条通知",
        Language::RO => "Sunt afișate doar ultimele 30 de notificări",
        Language::KO => "최근 30개의 알림만 표시됩니다",
        Language::PT_BR => "Apenas as últimas 30 notificações são exibidas",
    }
}
