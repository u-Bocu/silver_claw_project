import mediapipe as mp
import cv2

class hand_detector():
    def __init__(self, mode = False, maxHands = 1, detectionCon = 0.5, trackCon = 0.5) -> None:
        self._mode = mode
        self._maxHands = maxHands
        self._detectionCon = detectionCon
        self._trackCon = trackCon

        self._mpHands = mp.solutions.hands
        self._hands = self._mpHands.Hands(self._mode, self._maxHands, self._detectionCon, self._trackCon)
        self._mpDraw = mp.solutions.drawing_utils

    def findHands(self, img, draw = True):
        imgRGB = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
        self._results = self._hands.process(imgRGB)

        if self._results.multi_hand_landmarks:
            for handLms in self._results.multi_hand_landmarks:
                if draw:
                    self._mpDraw.draw_landmarks(img, handLms, self._mpHands.HAND_CONNECTIONS)
        return img

    def findPosition(self, img, handNo = 0, draw = True):
        lmList = []

        if self._results.multi_hand_landmarks:
            hand = self._results.multi_hand_landmarks[handNo]

            for id, lm in enumerate(hand.landmark):
                h, w, c = img.shape
                cx, cy = int(lm.x * w), int(lm.y * h)
                lmList.append([id, cx, cy])

                if draw:
                    cv2.circle(img, (cx, cy), 3, (255, 0, 255), cv2.FILLED)

        return lmList
