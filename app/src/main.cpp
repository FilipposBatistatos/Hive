#include <QApplication>
#include <QGraphicsView>
#include <QGraphicsScene>
#include <QPolygonF>
#include <cmath>

QPolygonF hexagon(QPointF centre, double size) {
    QPolygonF poly;
    for (int i = 0; i < 6; ++i) {
        double angle = M_PI / 180.0 * (60 * i - 30); // pointy-top
        poly << QPointF(centre.x() + size * std::cos(angle),
                         centre.y() + size * std::sin(angle));
    }
    return poly;
}

int main(int argc, char* argv[]) {
    QApplication app(argc, argv);

    QGraphicsScene scene;
    scene.addPolygon(hexagon({0, 0}, 40), QPen(Qt::black), QBrush(Qt::lightGray));

    QGraphicsView view(&scene);
    view.setRenderHint(QPainter::Antialiasing);
    view.resize(600, 600);
    view.show();

    return app.exec();
}
