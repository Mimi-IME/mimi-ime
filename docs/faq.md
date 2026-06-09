# Câu hỏi thường gặp

## Các dự án bộ gõ khác thì sao?

Dự án khác mình không thấy có dự án bộ gõ nào khác có tiềm năng nên rất tiếc mình không thể đề xuất
vì fcitx5 hay ibus đều là bọc từ 1 phần mềm lên tự tạo cho bản thân 1 input method nhưng nó rất khó
hiểu và trải nghiệm phát triển cũng như tài liệu rất tệ nên đây là lý do dự án này được sinh ra để có
thể dễ dàng kiểm soát dự án, có toolchain vui vẻ để phát triển và dễ theo chuẩn Wayland protocols.

Nhưng Mimi IME sinh ra vì mình cần nó chứ không phải ai cần nó cả, mình cần nó giúp tốt công việc của
mình. Nếu bạn thấy fcitx5 hay ibus cũng ổn thì cứ dùng vì suy cho cùng nó chỉ là bộ gõ tiếng Việt khác.

## Sẽ có hỗ trợ GNOME/KDE không?

Có hoặc không tùy thuộc vào tiến độ dự án và cảm hứng của mình. Đây là dự án tự mình làm cho mình nên
sẽ không có nhu cầu sử dụng GNOME/KDE trong tương lai gần. Nói về vấn đề input method wayland thì nó rất
hỗn loạn không phải tệ như mọi người thường nói nhưng vì mỗi bên tự tạo ra tiêu chuẩn nên dự án này
sẽ chỉ hỗ trợ được phần nào đó thôi.

Hãy tham khảo thêm ở [input-method-unstable-v2](https://wayland.app/protocols/input-method-unstable-v2)

GNOME/KDE không implement input_method_v2 nên buộc các dự án như fcitx5/ibus phải tự viết backend riêng cho từng DE. Developer IME độc lập không có đủ nguồn lực làm vậy.

Lý do GNOME/KDE đưa ra là protocol chưa ổn định — nhưng protocol không ổn định vì ít người implement,
ít người implement vì DE lớn không dùng. Vòng lặp con gà và quả trứng hoàn hảo.

Nếu GNOME/KDE không implement input_method_v2 thì Mimi-IME cũng sẽ không support họ trong tương lai gần
— đơn giản vậy thôi. Muốn dùng Mimi-IME thì đổi sang compositor hỗ trợ chuẩn Wayland như Niri, Sway, Hyprland có hỗ trợ input-method-unstable-v2.

Chính vì vậy các dự án bộ gõ khác bọc fcitx5 hay ibus không phải là bỏ xó chỉ là với cá nhân mình dự án Mimi IME
giảm tải 1 đống phụ thuộc vào GTK/QT chỉ tập trung cho các WM khác thôi.

Thay vào đó hãy cổ vũ các dự án bộ gõ tiếng Việt trên fcitx5/ibus — những người thích cosplay chú lùn (GNOME) và rồng K (KDE)
thì đừng mê Mimi IME. 😊❤️

## Sẽ có hỗ trợ Xorg chứ?

Không. Ít nhất ở thời điểm hiện tại không có lý do nào để chuyển sang Xorg cả nên dự án ưu tiên Wayland. Nhưng tuỳ vào Xorg
có còn tốt hơn Wayland không hay là 1 codebase không thể duy trì đầy lỗi bảo mật. Xlibre? không một ai biết thế giới sẽ chao đảo
thế nào nữa.

## Được sử dụng AI để code chứ?

Được và dự án này mình cũng xài AI để code. Thứ duy nhất mình quan tâm đó là nó phải hợp lý đúng logic code ít nhất cũng phải dễ hiểu
không được tào lao nếu nó không biên dịch được. Nếu PR bạn quá rác thì ít nhất hãy làm sao cho nó hoạt động. Còn nếu bạn chỉ trích AI slop
xin lỗi bạn có quyền tạo dự án mới và đọc các tiêu chuẩn để dành nhiều thời gian hơn cho dự án của bạn. Mình không quan tâm miễn sao mình
có thể gõ tiếng Việt.
